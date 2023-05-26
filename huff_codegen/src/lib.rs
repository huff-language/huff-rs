#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use huff_utils::{
    abi::*,
    artifact::*,
    ast::*,
    bytecode::*,
    bytes_util,
    error::CodegenError,
    evm::Opcode,
    prelude::{format_even_bytes, pad_n_bytes, CodegenErrorKind, EVMVersion, FileSource, Span},
    types::EToken,
};
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, fs, path::Path, sync::Arc};

mod irgen;
use crate::irgen::prelude::*;

/// ### Codegen
///
/// Code Generation Manager responsible for generating bytecode from a
/// [Contract](../../huff_utils/src/ast.rs#Contract) Abstract Syntax Tree.
///
/// #### Usage
///
/// The canonical way to instantiate a Codegen instance is using the public associated
/// [new](Codegen::new) function.
///
///
/// ```rust
/// use huff_codegen::Codegen;
/// let cg = Codegen::new();
/// ```
#[derive(Debug, Default, Clone)]
pub struct Codegen {
    /// The Input AST
    pub ast: Option<Contract>,
    /// A cached codegen output artifact
    pub artifact: Option<Artifact>,
    /// Intermediate main bytecode store
    pub main_bytecode: Option<String>,
    /// Intermediate constructor bytecode store
    pub constructor_bytecode: Option<String>,
}

impl Codegen {
    /// Public associated function to instantiate a new Codegen instance.
    pub fn new() -> Self {
        Self { ast: None, artifact: None, main_bytecode: None, constructor_bytecode: None }
    }

    /// Generates main bytecode from a Contract AST
    pub fn generate_main_bytecode(
        evm_version: &EVMVersion,
        contract: &Contract,
        alternative_main: Option<String>,
    ) -> Result<String, CodegenError> {
        // If an alternative main is provided, then use it as the compilation target
        let main_macro = alternative_main.unwrap_or_else(|| String::from("MAIN"));

        // Find the main macro
        let m_macro = Codegen::get_macro_by_name(&main_macro, contract)?;

        // For each MacroInvocation Statement, recurse into bytecode
        let bytecode_res: BytecodeRes = Codegen::macro_to_bytecode(
            evm_version,
            m_macro,
            contract,
            &mut vec![m_macro],
            0,
            &mut Vec::default(),
            false,
            None,
        )?;

        tracing::debug!(target: "codegen", "Generated main bytecode. Appending table bytecode...");

        // Generate the fully baked bytecode
        Codegen::gen_table_bytecode(bytecode_res)
    }

    /// Generates constructor bytecode from a Contract AST
    pub fn generate_constructor_bytecode(
        evm_version: &EVMVersion,
        contract: &Contract,
        alternative_constructor: Option<String>,
    ) -> Result<(String, bool), CodegenError> {
        // If an alternative constructor macro is provided, then use it as the compilation target
        let constructor_macro =
            alternative_constructor.unwrap_or_else(|| String::from("CONSTRUCTOR"));

        // Find the constructor macro
        let c_macro = Codegen::get_macro_by_name(&constructor_macro, contract)?;

        // For each MacroInvocation Statement, recurse into bytecode
        let bytecode_res: BytecodeRes = Codegen::macro_to_bytecode(
            evm_version,
            c_macro,
            contract,
            &mut vec![c_macro],
            0,
            &mut Vec::default(),
            false,
            None,
        )?;

        // Check if the constructor performs its own code generation
        let has_custom_bootstrap = bytecode_res.bytes.iter().any(|bytes| bytes.1 .0 == *"f3");

        tracing::info!(target: "codegen", "Constructor is self-generating: {}", has_custom_bootstrap);

        let bytecode = Codegen::gen_table_bytecode(bytecode_res)?;

        Ok((bytecode, has_custom_bootstrap))
    }

    /// Helper function to find a macro or generate a CodegenError
    pub(crate) fn get_macro_by_name<'a>(
        name: &str,
        contract: &'a Contract,
    ) -> Result<&'a MacroDefinition, CodegenError> {
        if let Some(m) = contract.find_macro_by_name(name) {
            Ok(m)
        } else {
            tracing::error!(target: "codegen", "MISSING \"{}\" MACRO!", name);
            Err(CodegenError {
                kind: CodegenErrorKind::MissingMacroDefinition(name.to_string()),
                span: AstSpan(vec![Span { start: 0, end: 0, file: None }]),
                token: None,
            })
        }
    }

    /// Appends table bytecode to the end of the BytecodeRes output.
    /// Fills table JUMPDEST placeholders.
    pub fn gen_table_bytecode(res: BytecodeRes) -> Result<String, CodegenError> {
        if !res.unmatched_jumps.is_empty() {
            tracing::error!(
                target: "codegen",
                "Source contains unmatched jump labels \"{}\"",
                res.unmatched_jumps.iter().map(|uj| uj.label.to_string()).collect::<Vec<String>>().join(", ")
            );
            return Err(CodegenError {
                kind: CodegenErrorKind::UnmatchedJumpLabel,
                span: AstSpan(
                    res.unmatched_jumps
                        .iter()
                        .flat_map(|uj| uj.span.0.clone())
                        .collect::<Vec<Span>>(),
                ),
                token: None,
            })
        }

        tracing::info!(target: "codegen", "GENERATING JUMPTABLE BYTECODE");

        let mut bytecode = res.bytes.into_iter().map(|(_, b)| b.0).collect::<String>();
        let mut table_offsets: HashMap<String, usize> = HashMap::new(); // table name -> bytecode offset
        let mut table_offset = bytecode.len() / 2;

        res.utilized_tables.iter().try_for_each(|jt| {
            table_offsets.insert(jt.name.to_string(), table_offset);
            let size = match bytes_util::hex_to_usize(bytes_util::bytes32_to_string(&jt.size, false).as_str()) {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!(target: "codegen", "Errored converting bytes32 to str. Bytes {:?} with error: {:?}", jt.size, e);
                    return Err(CodegenError {
                        kind: CodegenErrorKind::UsizeConversion(format!("{:?}", jt.size)),
                        span: jt.span.clone(),
                        token: None
                    })
                }
            };
            table_offset += size;

            tracing::info!(target: "codegen", "GENERATING BYTECODE FOR TABLE: \"{}\"", jt.name);

            let mut table_code = String::new();
            jt
                .statements
                .iter()
                .try_for_each(|s| {
                    match &s.ty {
                        StatementType::LabelCall(label) => {
                            let offset = match res.label_indices.get(label) {
                                Some(l) => l,
                                None => {
                                    tracing::error!(
                                    target: "codegen",
                                    "Definition not found for Jump Table Label: \"{}\"",
                                    label
                                );
                                    return Err(CodegenError {
                                        kind: CodegenErrorKind::UnmatchedJumpLabel,
                                        span: s.span.clone(),
                                        token: None,
                                    });
                                }
                            };
                            let hex = format_even_bytes(format!("{offset:02x}"));

                            table_code = format!("{table_code}{}", pad_n_bytes(
                                hex.as_str(),
                                if matches!(jt.kind, TableKind::JumpTablePacked) { 0x02 } else { 0x20 },
                            ));
                        }
                        StatementType::Code(code) => {
                            // Check if code length is even
                            if code.len() % 2 != 0 {
                                return Err(CodegenError {
                                    kind: CodegenErrorKind::InvalidCodeLength(code.len()),
                                    span: s.span.clone(),
                                    token: None,
                                });
                            }

                            table_code = format!("{table_code}{code}");
                        }
                        _ => {
                            return Err(CodegenError {
                                kind: CodegenErrorKind::InvalidMacroStatement,
                                span: jt.span.clone(),
                                token: None
                            })
                        }
                    }
                    Ok(())
                })?;
            tracing::info!(target: "codegen", "SUCCESSFULLY GENERATED BYTECODE FOR TABLE: \"{}\"", jt.name);
            bytecode = format!("{bytecode}{table_code}");
            Ok(())
        })?;

        res.table_instances.iter().for_each(|jump| {
            if let Some(o) = table_offsets.get(&jump.label) {
                let before = &bytecode[0..jump.bytecode_index * 2 + 2];
                let after = &bytecode[jump.bytecode_index * 2 + 6..];

                bytecode =
                    format!("{before}{}{after}", pad_n_bytes(format!("{o:02x}").as_str(), 2));
                tracing::info!(target: "codegen", "FILLED JUMPDEST FOR LABEL \"{}\"", jump.label);
            } else {
                tracing::error!(
                    target: "codegen",
                    "Jump table offset not present for jump label \"{}\"",
                    jump.label
                );
            }
        });

        Ok(bytecode)
    }

    /// Recurses a MacroDefinition to generate Bytecode
    ///
    /// ## Overview
    ///
    /// `macro_to_bytecode` first transforms the macro definition into "IR" Bytecode - a vec of
    /// intermediate bytes. It then iterates over each byte, converting the
    /// [IRByte](struct.IRByte.html) into a `Bytes`. Once done iterating over the macro
    /// definition IRBytes, we use the JumpTable to match any unmatched jumps. If jumps are not
    /// matched, they are appended to a vec of unmatched jumps.
    ///
    /// On success, a [BytecodeRes](struct.BytecodeRes.html) is returned,
    /// containing the generated bytes, label indices, unmatched jumps, and table indices.
    ///
    /// ## Arguments
    ///
    /// * `macro_def` - Macro definition to convert to bytecode
    /// * `contract` - Reference to the `Contract` AST generated by the parser
    /// * `scope` - Current scope of the recursion. Contains all macro definitions recursed so far.
    /// * `offset` - Current bytecode offset
    /// * `mis` - Vector of tuples containing parent macro invocations as well as their offsets.
    #[allow(clippy::too_many_arguments)]
    pub fn macro_to_bytecode<'a>(
        evm_version: &EVMVersion,
        macro_def: &'a MacroDefinition,
        contract: &'a Contract,
        scope: &mut Vec<&'a MacroDefinition>,
        mut offset: usize,
        mis: &mut Vec<(usize, MacroInvocation)>,
        recursing_constructor: bool,
        circular_codesize_invocations: Option<&mut CircularCodeSizeIndices>,
    ) -> Result<BytecodeRes, CodegenError> {
        // Get intermediate bytecode representation of the macro definition
        let mut bytes: Vec<(usize, Bytes)> = Vec::default();
        let ir_bytes = macro_def.to_irbytecode(evm_version)?.0;

        // Define outer loop variables
        let mut jump_table = JumpTable::new();
        let mut label_indices = LabelIndices::new();
        let mut table_instances = Jumps::new();
        let mut utilized_tables: Vec<TableDefinition> = Vec::new();
        let mut ccsi = CircularCodeSizeIndices::new();
        let circular_codesize_invocations = circular_codesize_invocations.unwrap_or(&mut ccsi);

        // Loop through all intermediate bytecode representations generated from the AST
        for (_ir_bytes_index, ir_byte) in ir_bytes.iter().enumerate() {
            let starting_offset = offset;
            match &ir_byte.ty {
                IRByteType::Bytes(b) => {
                    offset += b.0.len() / 2;
                    bytes.push((starting_offset, b.to_owned()));
                }
                IRByteType::Constant(name) => {
                    let push_bytes = constant_gen(evm_version, name, contract, ir_byte.span)?;
                    offset += push_bytes.len() / 2;
                    tracing::debug!(target: "codegen", "OFFSET: {}, PUSH BYTES: {:?}", offset, push_bytes);
                    bytes.push((starting_offset, Bytes(push_bytes)));
                }
                IRByteType::Statement(s) => {
                    // if we have a codesize call for the constructor here, from within the
                    // constructor, we skip
                    if recursing_constructor {
                        continue
                    }
                    let mut push_bytes = statement_gen(
                        evm_version,
                        s,
                        contract,
                        macro_def,
                        scope,
                        &mut offset,
                        mis,
                        &mut jump_table,
                        &mut label_indices,
                        &mut table_instances,
                        &mut utilized_tables,
                        circular_codesize_invocations,
                        starting_offset,
                    )?;
                    bytes.append(&mut push_bytes);
                }
                IRByteType::ArgCall(arg_name) => {
                    // Bubble up arg call by looking through the previous scopes.
                    // Once the arg value is found, add it to `bytes`
                    bubble_arg_call(
                        arg_name,
                        &mut bytes,
                        macro_def,
                        contract,
                        scope,
                        &mut offset,
                        mis,
                        &mut jump_table,
                    )?
                }
            }
        }

        // We're done, let's pop off the macro invocation
        if mis.pop().is_none() {
            tracing::warn!(target: "codegen", "ATTEMPTED MACRO INVOCATION POP FAILED AT SCOPE: {}", scope.len());
        }

        // Add functions (outlined macros) to the end of the bytecode if the scope length == 1
        // (i.e., we're at the top level of recursion)
        if scope.len() == 1 {
            bytes = Codegen::append_functions(
                evm_version,
                contract,
                scope,
                &mut offset,
                mis,
                &mut jump_table,
                &mut label_indices,
                &mut table_instances,
                bytes,
            )?;
        } else {
            // If the scope length is > 1, we're processing a child macro. Since we're done
            // with it, it can be popped.
            scope.pop();
        }

        // Fill JUMPDEST placeholders
        let (bytes, unmatched_jumps) = Codegen::fill_unmatched(bytes, &jump_table, &label_indices)?;

        // Fill in circular codesize invocations
        // Workout how to increase the offset the correct amount within here if it is longer than 2
        // bytes
        let bytes = Codegen::fill_circular_codesize_invocations(
            bytes,
            circular_codesize_invocations,
            &macro_def.name,
        )?;

        Ok(BytecodeRes { bytes, label_indices, unmatched_jumps, table_instances, utilized_tables })
    }

    /// Helper associated function to fill unmatched jump dests.
    ///
    /// ## Overview
    ///
    /// Iterates over the vec of generated bytes. At each index, check if a jump is tracked.
    /// If one is, find the index of label and inplace the formatted location.
    /// If there is no label matching the jump, we append the jump to a list of unmatched jumps,
    /// updating the jump's bytecode index.
    ///
    /// On success, returns a tuple of generated bytes and unmatched jumps.
    /// On failure, returns a CodegenError.
    #[allow(clippy::type_complexity)]
    pub fn fill_unmatched(
        bytes: Vec<(usize, Bytes)>,
        jump_table: &JumpTable,
        label_indices: &LabelIndices,
    ) -> Result<(Vec<(usize, Bytes)>, Vec<Jump>), CodegenError> {
        let mut unmatched_jumps = Jumps::default();
        let bytes =
            bytes.into_iter().fold(Vec::default(), |mut acc, (code_index, mut formatted_bytes)| {
                // Check if a jump table exists at `code_index` (starting offset of `b`)
                if let Some(jt) = jump_table.get(&code_index) {
                    // Loop through jumps inside of the found JumpTable
                    for jump in jt {
                        // Check if the jump label has been defined. If not, add `jump` to the
                        // unmatched jumps and define its `bytecode_index`
                        // at `code_index`
                        if let Some(jump_index) = label_indices.get(jump.label.as_str()) {
                            // Format the jump index as a 2 byte hex number
                            let jump_value = format!("{jump_index:04x}");

                            // Get the bytes before & after the placeholder
                            let before = &formatted_bytes.0[0..jump.bytecode_index + 2];
                            let after = &formatted_bytes.0[jump.bytecode_index + 6..];

                            // Check if a jump dest placeholder is present
                            if !&formatted_bytes.0[jump.bytecode_index + 2..jump.bytecode_index + 6]
                                .eq("xxxx")
                            {
                                tracing::error!(
                                    target: "codegen",
                                    "JUMP DESTINATION PLACEHOLDER NOT FOUND FOR JUMPLABEL {}",
                                    jump.label
                                );
                            }

                            // Replace the "xxxx" placeholder with the jump value
                            formatted_bytes = Bytes(format!("{before}{jump_value}{after}"));
                        } else {
                            // The jump did not have a corresponding label index. Add it to the
                            // unmatched jumps vec.
                            unmatched_jumps.push(Jump {
                                label: jump.label.clone(),
                                bytecode_index: code_index,
                                span: jump.span.clone(),
                            });
                        }
                    }
                }

                acc.push((code_index, formatted_bytes));
                acc
            });

        Ok((bytes, unmatched_jumps))
    }

    /// Helper associated function to fill circular codesize invocations.
    ///
    /// ## Overview
    ///
    /// This function should run after all other code generation has been completed.
    /// If there are placeholders for circular codesize invocations, this function will
    /// fill them in with the correct offset.
    ///
    /// If there are multiple invocations of the same macro, the function will take into
    /// account the total number of invocations and increase its offset accordingly.
    ///
    /// On success, returns a tuple of generated bytes.
    /// On failure, returns a CodegenError.
    pub fn fill_circular_codesize_invocations(
        bytes: Vec<(usize, Bytes)>,
        circular_codesize_invocations: &mut CircularCodeSizeIndices,
        macro_name: &str,
    ) -> Result<Vec<(usize, Bytes)>, CodegenError> {
        // Get the length of the macro
        let num_invocations = circular_codesize_invocations.len();
        if num_invocations == 0 {
            return Ok(bytes)
        }

        tracing::debug!(target: "codegen", "Circular Codesize Invocation: Bytes before expansion: {:#?}", bytes);
        let length: usize = bytes.iter().map(|(_, b)| b.0.len()).sum::<usize>() / 2;

        // If there are more than 256 opcodes in a macro, we need 2 bytes to represent it
        // The next threshold is 65536 opcodes which is past the codesize limit
        let mut offset_increase = 0;
        if length > 255 {
            offset_increase = 1;
        }
        // Codesize will increase by 1 byte for every codesize that exists
        let extended_length = length + (offset_increase * num_invocations);

        let push_size = format_even_bytes(format!("{extended_length:02x}"));
        let push_bytes = format!("{:02x}{push_size}", 95 + push_size.len() / 2);

        // Track the number of bytes added if there is an offset increase with codesize
        let mut running_increase = 0;
        let bytes = bytes.into_iter().fold(
            Vec::default(),
            |mut acc, (mut code_index, mut formatted_bytes)| {
                // Check if a jump table exists at `code_index` (starting offset of `b`)
                if let Some((_, _index)) =
                    circular_codesize_invocations.get(&(macro_name.to_string(), code_index))
                {
                    // Check if a jump dest placeholder is present
                    if !&formatted_bytes.0.eq("cccc") {
                        tracing::error!(
                            target: "codegen",
                            "CIRCULAR CODESIZE PLACEHOLDER NOT FOUND"
                        );
                    }

                    // Replace the "cccc" placeholder with the jump value
                    formatted_bytes = Bytes(push_bytes.to_string());
                    running_increase += offset_increase;
                } else {
                    // Increase the code index by the number of bytes added past the placeholder
                    code_index += running_increase;
                }

                acc.push((code_index, formatted_bytes));
                acc
            },
        );

        Ok(bytes)
    }

    /// Helper associated function to append functions to the end of the bytecode.
    ///
    /// ## Overview
    ///
    /// Iterates over the contract's functions, generates their bytecode, fills unmatched jumps &
    /// label indices, and appends the functions' bytecode to the end of the contract's bytecode.
    ///
    /// On success, passes ownership of `bytes` back to the caller.
    /// On failure, returns a CodegenError.
    #[allow(clippy::too_many_arguments)]
    pub fn append_functions<'a>(
        evm_version: &EVMVersion,
        contract: &'a Contract,
        scope: &mut Vec<&'a MacroDefinition>,
        offset: &mut usize,
        mis: &mut Vec<(usize, MacroInvocation)>,
        jump_table: &mut JumpTable,
        label_indices: &mut LabelIndices,
        table_instances: &mut Jumps,
        mut bytes: Vec<(usize, Bytes)>,
    ) -> Result<Vec<(usize, Bytes)>, CodegenError> {
        for macro_def in contract.macros.iter().filter(|m| m.outlined) {
            // Push the function to the scope
            scope.push(macro_def);

            // Add 1 to starting offset to account for the JUMPDEST opcode
            let mut res = Codegen::macro_to_bytecode(
                evm_version,
                macro_def,
                contract,
                scope,
                *offset + 1,
                mis,
                false,
                None,
            )?;

            for j in res.unmatched_jumps.iter_mut() {
                let new_index = j.bytecode_index;
                j.bytecode_index = 0;
                let mut new_jumps = if let Some(jumps) = jump_table.get(&new_index) {
                    jumps.clone()
                } else {
                    vec![]
                };
                new_jumps.push(j.clone());
                jump_table.insert(new_index, new_jumps);
            }
            table_instances.extend(res.table_instances);
            label_indices.extend(res.label_indices);

            let macro_code_len = res.bytes.iter().map(|(_, b)| b.0.len()).sum::<usize>() / 2;

            // Get necessary swap ops to reorder stack
            // PC of the return jumpdest should be above the function's outputs on the stack
            let stack_swaps =
                (0..macro_def.returns).map(|i| format!("{:02x}", 0x90 + i)).collect::<Vec<_>>();

            // Insert JUMPDEST, stack swaps, and final JUMP back to the location of invocation.
            bytes.push((*offset, Bytes(Opcode::Jumpdest.to_string())));
            res.bytes.push((
                *offset + macro_code_len + 1,
                Bytes(format!("{}{}", stack_swaps.join(""), Opcode::Jump)),
            ));
            bytes = [bytes, res.bytes].concat();
            // Add the jumpdest to the beginning of the outlined macro.
            label_indices.insert(format!("goto_{}", macro_def.name.clone()), *offset);
            *offset += macro_code_len + stack_swaps.len() + 2; // JUMPDEST + MACRO_CODE_LEN +
                                                               // stack_swaps.len() + JUMP
        }
        Ok(bytes)
    }

    /// Generate a codegen artifact
    ///
    /// # Arguments
    ///
    /// * `args` - A vector of Tokens representing constructor arguments
    /// * `main_bytecode` - The compiled MAIN Macro bytecode
    /// * `constructor_bytecode` - The compiled `CONSTRUCTOR` Macro bytecode
    pub fn churn(
        &mut self,
        file: Arc<FileSource>,
        mut args: Vec<ethers_core::abi::token::Token>,
        main_bytecode: &str,
        constructor_bytecode: &str,
        has_custom_bootstrap: bool,
    ) -> Result<Artifact, CodegenError> {
        let artifact: &mut Artifact = if let Some(art) = &mut self.artifact {
            art
        } else {
            self.artifact = Some(Artifact::default());
            self.artifact.as_mut().unwrap()
        };

        // Move `main_bytecode` to the heap so that it can be modified if need be.
        let mut main_bytecode = String::from(main_bytecode);

        let contract_length = main_bytecode.len() / 2;
        let constructor_length = constructor_bytecode.len() / 2;

        // Sort constructor arguments so that statically sized args are inserted last.
        args.sort_by(|a, b| {
            if a.is_dynamic() && !b.is_dynamic() {
                Ordering::Less
            } else if !a.is_dynamic() && b.is_dynamic() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        let mut arg_offset_acc = contract_length;
        let encoded: Vec<Vec<u8>> = args
            .into_iter()
            .enumerate()
            .map(|(i, tok)| {
                if tok.is_dynamic() {
                    let encoded = ethers_core::abi::encode(&[tok]);

                    // Check for "__CODECOPY_DYN_ARG" calls for this specific argument. If any
                    // exist, fill the placeholders.
                    let tok_len = hex::encode(&encoded[62..64]);
                    let rep_regex =
                        Regex::new(format!("xxxxxxxxxxxxxxxxxxxxxxxxxxxx{i:02x}\\d{{4}}").as_str())
                            .unwrap();
                    rep_regex.find_iter(main_bytecode.clone().as_str()).for_each(|s| {
                        // TODO: Enforce that the arg type is a literal so that this unwrap is safe.
                        let len_ptr = usize::from_str_radix(&s.as_str()[30..34], 16).unwrap();
                        let contents_ptr = len_ptr + 0x20;

                        // Replace 17 reserved bytes.
                        main_bytecode.replace_range(
                            s.range(),
                            format!(
                                "{}{}{}{:04x}{}{}{}{}{:04x}{}{:04x}{}",
                                Opcode::Push2,    // PUSH2
                                &tok_len,         // len(bytes)
                                Opcode::Push2,    // PUSH2
                                len_ptr,          // <len_mem_ptr>
                                Opcode::Mstore,   // MSTORE
                                Opcode::Push2,    // PUSH2
                                &tok_len,         // len(bytes)
                                Opcode::Push2,    // PUSH2
                                arg_offset_acc,   // <contents_code_ptr>
                                Opcode::Push2,    // PUSH2
                                contents_ptr,     // <contents_mem_ptr>
                                Opcode::Codecopy  // CODECOPY
                            )
                            .as_str(),
                        );
                    });

                    // Increase argument offset accumulator.
                    arg_offset_acc += encoded.len() - 64;

                    // We don't need to store the pointer nor the length of dynamically sized
                    // elements in the code.
                    encoded[64..].into()
                } else {
                    ethers_core::abi::encode(&[tok])
                }
            })
            .collect();
        let hex_args: Vec<String> = encoded.iter().map(|tok| hex::encode(tok.as_slice())).collect();
        let constructor_args = hex_args.join("");

        // Sucks that we can't provide a span on this error. Need to refactor at some point.
        if main_bytecode.contains('x') {
            tracing::error!(target="codegen", "Failed to fill `__CODECOPY_DYN_ARG` placeholders. Dynamic argument index is invalid.");
            return Err(CodegenError {
                kind: CodegenErrorKind::InvalidDynArgIndex,
                span: AstSpan(vec![Span { start: 0, end: 0, file: None }]),
                token: None,
            })
        }

        // Constructor size optimizations
        let mut bootstrap_code_size = 9;
        let contract_size = if contract_length < 256 {
            format!("60{}", pad_n_bytes(format!("{contract_length:x}").as_str(), 1))
        } else {
            bootstrap_code_size += 1;

            format!("61{}", pad_n_bytes(format!("{contract_length:x}").as_str(), 2))
        };
        let contract_code_offset = if (bootstrap_code_size + constructor_length) < 256 {
            format!(
                "60{}",
                pad_n_bytes(format!("{:x}", bootstrap_code_size + constructor_length).as_str(), 1)
            )
        } else {
            bootstrap_code_size += 1;

            format!(
                "61{}",
                pad_n_bytes(format!("{:x}", bootstrap_code_size + constructor_length).as_str(), 2)
            )
        };

        let bootstrap_code = if has_custom_bootstrap {
            String::default()
        } else {
            format!("{contract_size}80{contract_code_offset}3d393df3")
        };

        // Generate the final bytecode
        let constructor_code = format!("{constructor_bytecode}{bootstrap_code}");
        artifact.bytecode =
            format!("{constructor_code}{main_bytecode}{constructor_args}").to_lowercase();
        artifact.runtime = main_bytecode.to_lowercase();
        artifact.file = file;
        Ok(artifact.clone())
    }

    /// Encode constructor arguments as ethers_core::abi::token::Token
    pub fn encode_constructor_args(args: Vec<String>) -> Vec<ethers_core::abi::token::Token> {
        let tokens: Vec<ethers_core::abi::token::Token> =
            args.iter().map(|tok| EToken::try_from(tok.clone()).unwrap().0).collect();
        tokens
    }

    /// Export
    ///
    /// Writes a Codegen Artifact out to the specified file.
    ///
    /// # Arguments
    ///
    /// * `out` - Output location to write the serialized json artifact to.
    pub fn export(output: String, art: &Artifact) -> Result<(), CodegenError> {
        let serialized_artifact = serde_json::to_string_pretty(art).unwrap();
        // Try to create the parent directory
        let file_path = Path::new(&output);
        if let Some(p) = file_path.parent() {
            if let Err(e) = fs::create_dir_all(p) {
                return Err(CodegenError {
                    kind: CodegenErrorKind::IOError(e.to_string()),
                    span: AstSpan(vec![Span {
                        start: 0,
                        end: 0,
                        file: Some(Arc::new(FileSource {
                            id: uuid::Uuid::new_v4(),
                            path: output,
                            source: None,
                            access: None,
                            dependencies: None,
                        })),
                    }]),
                    token: None,
                })
            }
        }
        if let Err(e) = fs::write(file_path, serialized_artifact) {
            return Err(CodegenError {
                kind: CodegenErrorKind::IOError(e.to_string()),
                span: AstSpan(vec![Span {
                    start: 0,
                    end: 0,
                    file: Some(Arc::new(FileSource {
                        id: uuid::Uuid::new_v4(),
                        path: output,
                        source: None,
                        access: None,
                        dependencies: None,
                    })),
                }]),
                token: None,
            })
        }
        Ok(())
    }

    /// Abi Generation
    ///
    /// Generates an ABI for the given Ast.
    /// Stores the generated ABI in the Codegen `artifact`.
    ///
    /// # Arguments
    ///
    /// * `ast` - The Contract Abstract Syntax Tree
    /// * `output` - An optional output path
    pub fn abi_gen(&mut self, ast: Contract, output: Option<String>) -> Result<Abi, CodegenError> {
        let abi: Abi = ast.into();

        // Set the abi on self
        let art: &Artifact = match &mut self.artifact {
            Some(artifact) => {
                artifact.abi = Some(abi.clone());
                artifact
            }
            None => {
                self.artifact = Some(Artifact { abi: Some(abi.clone()), ..Default::default() });
                self.artifact.as_ref().unwrap()
            }
        };

        // If an output's specified, write the artifact out
        if let Some(o) = output {
            // Error message is sent to tracing in `export` if an error occurs
            Codegen::export(o, art)?;
        }

        // Return the abi
        Ok(abi)
    }
}
