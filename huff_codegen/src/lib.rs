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
    error::CodegenError,
    evm::Opcode,
    prelude::{
        bytes32_to_string, format_even_bytes, pad_n_bytes, CodegenErrorKind, FileSource, Span,
    },
    types::EToken,
};
use std::{collections::HashMap, fs, path::Path, sync::Arc};

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
#[derive(Debug, Default, PartialEq, Eq, Clone)]
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
    pub fn generate_main_bytecode(contract: &Contract) -> Result<String, CodegenError> {
        // Find the main macro
        let m_macro = Codegen::get_macro_by_name("MAIN", contract)?;

        // For each MacroInvocation Statement, recurse into bytecode
        let bytecode_res: BytecodeRes = Codegen::macro_to_bytecode(
            m_macro.clone(),
            contract,
            &mut vec![m_macro],
            0,
            &mut Vec::default(),
        )?;

        // Generate the fully baked bytecode
        Codegen::gen_table_bytecode(bytecode_res, contract)
    }

    /// Generates constructor bytecode from a Contract AST
    pub fn generate_constructor_bytecode(contract: &Contract) -> Result<String, CodegenError> {
        // Find the constructor macro
        let c_macro = Codegen::get_macro_by_name("CONSTRUCTOR", contract)?;

        // For each MacroInvocation Statement, recurse into bytecode
        let bytecode_res: BytecodeRes = Codegen::macro_to_bytecode(
            c_macro.clone(),
            contract,
            &mut vec![c_macro],
            0,
            &mut Vec::default(),
        )?;

        // Generate the bytecode return string
        Codegen::gen_table_bytecode(bytecode_res, contract)
    }

    /// Helper function to find a macro or generate a CodegenError
    pub(crate) fn get_macro_by_name(
        name: &str,
        contract: &Contract,
    ) -> Result<MacroDefinition, CodegenError> {
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
    pub(crate) fn gen_table_bytecode(
        res: BytecodeRes,
        contract: &Contract,
    ) -> Result<String, CodegenError> {
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

        if let Err(e) = contract.tables.iter().try_for_each(|jt| {
            table_offsets.insert(jt.name.to_string(), table_offset);
            let size = match bytes32_to_string(&jt.size, false).parse::<usize>() {
                Ok(s) => s,
                Err(_) => return Err(CodegenError {
                    kind: CodegenErrorKind::UsizeConversion(format!("{:?}", jt.size)),
                    span: jt.span.clone(),
                    token: None
                })
            };
            table_offset += size;

            tracing::info!(target: "codegen", "GENERATING BYTECODE FOR TABLE: \"{}\"", jt.name);

            let mut table_code = String::new();
            let collected = jt
                .statements
                .iter()
                .try_for_each(|s| {
                    if let StatementType::LabelCall(label) = &s.ty {
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
                        let hex = format_even_bytes(format!("{:02x}", offset));

                        table_code = format!("{}{}", table_code, pad_n_bytes(
                            hex.as_str(),
                            if matches!(jt.kind, TableKind::JumpTablePacked) { 0x02 } else { 0x20 },
                        ));
                    }
                    Ok(())
                });
            if let Err(e) = collected {
                return Err(e);
            }
            tracing::info!(target: "codegen", "SUCCESSFULLY GENERATED BYTECODE FOR TABLE: \"{}\"", jt.name);
            bytecode = format!("{}{}", bytecode, table_code);
            Ok(())
        }) {
            return Err(e);
        }

        res.table_instances.iter().for_each(|jump| {
            if let Some(o) = table_offsets.get(&jump.label) {
                let before = &bytecode[0..jump.bytecode_index * 2 + 2];
                let after = &bytecode[jump.bytecode_index * 2 + 6..];

                bytecode =
                    format!("{}{}{}", before, pad_n_bytes(format!("{:02x}", o).as_str(), 2), after);
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
    pub(crate) fn macro_to_bytecode(
        macro_def: MacroDefinition,
        contract: &Contract,
        scope: &mut Vec<MacroDefinition>,
        mut offset: usize,
        mis: &mut Vec<(usize, MacroInvocation)>,
    ) -> Result<BytecodeRes, CodegenError> {
        // Get intermediate bytecode representation of the macro definition
        let mut bytes: Vec<(usize, Bytes)> = Vec::default();
        let ir_bytes = macro_def.to_irbytecode()?.0;

        // Define outer loop variables
        let mut jump_table = JumpTable::new();
        let mut label_indices = LabelIndices::new();
        let mut table_instances = Jumps::new();

        // Loop through all intermediate bytecode representations generated from the AST
        for (_ir_bytes_index, ir_byte) in ir_bytes.into_iter().enumerate() {
            let starting_offset = offset;
            match ir_byte.ty {
                IRByteType::Bytes(b) => {
                    offset += b.0.len() / 2;
                    bytes.push((starting_offset, b));
                }
                IRByteType::Constant(name) => {
                    let push_bytes = constant_gen(&name, contract, ir_byte.span)?;
                    offset += push_bytes.len() / 2;
                    tracing::debug!(target: "codegen", "OFFSET: {}, PUSH BYTES: {:?}", offset, push_bytes);
                    bytes.push((starting_offset, Bytes(push_bytes)));
                }
                IRByteType::Statement(s) => {
                    let mut push_bytes = statement_gen(
                        &s,
                        contract,
                        &macro_def,
                        scope,
                        &mut offset,
                        mis,
                        &mut jump_table,
                        &mut label_indices,
                        &mut table_instances,
                        starting_offset,
                    )?;
                    bytes.append(&mut push_bytes);
                }
                IRByteType::ArgCall(arg_name) => {
                    // Bubble up arg call by looking through the previous scopes.
                    // Once the arg value is found, add it to `bytes`
                    bubble_arg_call(
                        &arg_name,
                        &mut bytes,
                        &macro_def,
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

        // Add outlined macros to the end of the bytecode
        // TODO: Remove bad hack to detect end of recursion. Also possibly move this logic?
        if macro_def.name == "MAIN" {
            for macro_def in contract.macros.iter().filter(|m| m.outlined) {
                if let Ok(res) =
                    Codegen::macro_to_bytecode(macro_def.clone(), contract, scope, offset, mis)
                {
                    let macro_code_len =
                        res.bytes.iter().map(|(_, b)| b.0.len()).sum::<usize>() / 2;

                    // TODO: Clean up.
                    bytes = [
                        bytes,
                        vec![(offset, Bytes(Opcode::Jumpdest.to_string()))],
                        res.bytes,
                        vec![(
                            offset + macro_code_len + 1,
                            Bytes(format!("{}xxxx", Opcode::Push2)),
                        )],
                    ]
                    .concat();
                    // Add the jumpdest to the beginning of the outlined macro.
                    label_indices.insert(format!("goto_{}", macro_def.name.clone()), offset);
                    // Add the jump back to the position after it was called
                    jump_table.insert(
                        offset + macro_code_len + 1,
                        vec![Jump {
                            label: format!("return_from_{}", macro_def.name.clone()),
                            bytecode_index: 0,
                            span: macro_def.span.clone(), // TODO: Not the right span
                        }],
                    );
                    offset += macro_code_len + 4;
                } else {
                    tracing::error!(target: "codegen", "Failed to generate bytecode for macro: {}", macro_def.name);
                    return Err(CodegenError {
                        kind: CodegenErrorKind::MissingMacroDefinition(macro_def.name.clone()),
                        span: macro_def.span.clone(),
                        token: None,
                    })
                }
            }
        }

        // Fill JUMPDEST placeholders
        let (bytes, unmatched_jumps) = Codegen::fill_unmatched(bytes, &jump_table, &label_indices)?;

        Ok(BytecodeRes { bytes, label_indices, unmatched_jumps, table_instances })
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
                            let jump_value = format!("{:04x}", jump_index);

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
                            formatted_bytes = Bytes(format!("{}{}{}", before, jump_value, after));
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
        args: Vec<ethers_core::abi::token::Token>,
        main_bytecode: &str,
        constructor_bytecode: &str,
    ) -> Result<Artifact, CodegenError> {
        let mut artifact: &mut Artifact = if let Some(art) = &mut self.artifact {
            art
        } else {
            self.artifact = Some(Artifact::default());
            self.artifact.as_mut().unwrap()
        };

        let contract_length = main_bytecode.len() / 2;
        let constructor_length = constructor_bytecode.len() / 2;

        let encoded: Vec<Vec<u8>> =
            args.iter().map(|tok| ethers_core::abi::encode(&[tok.clone()])).collect();
        let hex_args: Vec<String> = encoded.iter().map(|tok| hex::encode(tok.as_slice())).collect();
        let constructor_args = hex_args.join("");

        // Constructor size optimizations
        let mut bootstrap_code_size = 9;
        let contract_size = if contract_length < 256 {
            format!("60{}", pad_n_bytes(format!("{:x}", contract_length).as_str(), 1))
        } else {
            bootstrap_code_size += 1;

            format!("61{}", pad_n_bytes(format!("{:x}", contract_length).as_str(), 2))
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

        // Generate the final bytecode
        let bootstrap_code = format!("{}80{}3d393df3", contract_size, contract_code_offset);
        let constructor_code = format!("{}{}", constructor_bytecode, bootstrap_code);
        artifact.bytecode =
            format!("{}{}{}", constructor_code, main_bytecode, constructor_args).to_lowercase();
        artifact.runtime = main_bytecode.to_string().to_lowercase();
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
            if let Err(e) = Codegen::export(o, art) {
                // Error message is sent to tracing in `export` if an error occurs
                return Err(e)
            }
        }

        // Return the abi
        Ok(abi)
    }
}
