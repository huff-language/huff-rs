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
    prelude::{bytes32_to_string, pad_n_bytes, CodegenErrorKind},
    types::EToken,
};
use std::fs;

/// ### Codegen
///
/// Code Generation Manager responsible for generating the code for the Huff Language.
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
    ///
    /// # Arguments
    ///
    /// * `ast` - Optional Contract Abstract Syntax Tree
    pub fn roll(ast: Option<Contract>) -> Result<String, CodegenError> {
        // Grab the AST
        let contract = match &ast {
            Some(a) => a,
            None => {
                tracing::error!(target: "codegen", "MISSING BOTH STATEFUL AND PARAMETER AST!");
                return Err(CodegenError {
                    kind: CodegenErrorKind::MissingAst,
                    span: None,
                    token: None,
                })
            }
        };

        // Find the main macro
        let m_macro: MacroDefinition = if let Some(m) = contract.find_macro_by_name("MAIN") {
            m
        } else {
            tracing::error!(target: "codegen", "MISSING \"MAIN\" MACRO!");
            return Err(CodegenError {
                kind: CodegenErrorKind::MissingMacroDefinition("MAIN".to_string()),
                span: None,
                token: None,
            })
        };

        tracing::info!(target: "codegen", "MAIN MACRO FOUND: {:?}", m_macro);

        // For each MacroInvocation Statement, recurse into bytecode
        let bytecode_res: BytecodeRes =
            Codegen::recurse_bytecode(m_macro.clone(), ast, &mut vec![m_macro], 0, Vec::default())?;
        tracing::info!(target: "codegen", "RECURSED BYTECODE: {:?}", bytecode_res);
        let bytecode = bytecode_res.bytes.iter().map(|byte| byte.0.to_string()).collect();
        tracing::info!(target: "codegen", "FINAL BYTECODE: {:?}", bytecode);

        // Return
        Ok(bytecode)
    }

    /// Gracefully get the Contract AST
    pub fn graceful_ast_grab(&self, ast: Option<Contract>) -> Result<Contract, CodegenError> {
        match ast {
            Some(a) => Ok(a),
            None => match &self.ast {
                Some(a) => Ok(a.clone()),
                None => {
                    tracing::error!("Neither Codegen AST was set nor passed in as a parameter to Codegen::construct()!");
                    Err(CodegenError {
                        kind: CodegenErrorKind::MissingAst,
                        span: None,
                        token: None,
                    })
                }
            },
        }
    }

    /// Generates constructor bytecode from a Contract AST
    ///
    /// # Arguments
    ///
    /// * `ast` - Optional Contract Abstract Syntax Tree
    pub fn construct(ast: Option<Contract>) -> Result<String, CodegenError> {
        // Grab the AST
        let contract = match &ast {
            Some(a) => a,
            None => {
                tracing::error!(target: "codegen", "Neither Codegen AST was set nor passed in as a parameter to Codegen::construct()!");
                return Err(CodegenError {
                    kind: CodegenErrorKind::MissingAst,
                    span: None,
                    token: None,
                })
            }
        };

        // Find the constructor macro
        let c_macro: MacroDefinition = if let Some(m) = contract.find_macro_by_name("CONSTRUCTOR") {
            m
        } else {
            tracing::error!(target: "codegen", "'CONSTRUCTOR' Macro definition missing in AST!");
            return Err(CodegenError {
                kind: CodegenErrorKind::MissingConstructor,
                span: None,
                token: None,
            })
        };

        tracing::info!(target: "codegen", "CONSTRUCTOR MACRO FOUND: {:?}", c_macro);

        // For each MacroInvocation Statement, recurse into bytecode
        let bytecode_res: BytecodeRes =
            Codegen::recurse_bytecode(c_macro.clone(), ast, &mut vec![c_macro], 0, Vec::default())?;
        tracing::info!(target: "codegen", "RECURSED BYTECODE: {:?}", bytecode_res);
        let bytecode = bytecode_res.bytes.iter().map(|byte| byte.0.to_string()).collect();
        tracing::info!(target: "codegen", "FINAL BYTECODE: {:?}", bytecode);

        // Return
        Ok(bytecode)
    }

    /// Recurses a MacroDefinition to generate Bytecode
    pub fn recurse_bytecode(
        macro_def: MacroDefinition,
        ast: Option<Contract>,
        scope: &mut Vec<MacroDefinition>,
        mut offset: usize,
        jump_tables: Vec<JumpTable>,
    ) -> Result<BytecodeRes, CodegenError> {
        let mut final_bytes: Vec<Bytes> = vec![];

        tracing::info!(target: "codegen", "RECURSING MACRO DEFINITION");

        // Grab the AST
        let contract = match &ast {
            Some(a) => a,
            None => {
                tracing::error!(target: "codegen", "Neither Codegen AST was set nor passed in as a parameter to Codegen::construct()!");
                return Err(CodegenError {
                    kind: CodegenErrorKind::MissingAst,
                    span: None,
                    token: None,
                })
            }
        };

        // Generate the macro bytecode
        let irb = macro_def.to_irbytecode()?;
        tracing::info!(target: "codegen", "GENERATED IRBYTECODE: {:?}", irb);
        let irbz = irb.0;

        let mut jump_table = JumpTable::new();
        let mut jump_indices = JumpIndices::new();

        for (index, ir_byte) in irbz.iter().enumerate() {
            match ir_byte.clone() {
                IRByte::Bytes(b) => {
                    offset += 1;
                    final_bytes.push(b)
                }
                IRByte::Constant(name) => {
                    let constant = if let Some(m) = contract
                        .constants
                        .iter()
                        .filter(|const_def| const_def.name == name)
                        .cloned()
                        .collect::<Vec<ConstantDefinition>>()
                        .get(0)
                    {
                        m.clone()
                    } else {
                        tracing::error!(target: "codegen", "MISSING CONTRACT MACRO \"{}\"", name);

                        // TODO we should try and find the constant defined in other files here
                        return Err(CodegenError {
                            kind: CodegenErrorKind::MissingConstantDefinition,
                            span: None,
                            token: None,
                        })
                    };

                    tracing::info!(target: "codegen", "FOUND CONSTANT DEFINITION: {:?}", constant);

                    let push_bytes = match constant.value {
                        ConstVal::Literal(l) => {
                            let hex_literal: String = bytes32_to_string(&l, false);
                            format!("{:02x}{}", 95 + hex_literal.len() / 2, hex_literal)
                        }
                        ConstVal::FreeStoragePointer(fsp) => {
                            // If this is reached in codegen stage, the `derive_storage_pointers`
                            // method was not called on the AST.
                            tracing::error!(target: "codegen", "STORAGE POINTERS INCORRECTLY DERIVED FOR \"{:?}\"", fsp);
                            return Err(CodegenError {
                                kind: CodegenErrorKind::StoragePointersNotDerived,
                                span: None,
                                token: None,
                            })
                        }
                    };

                    offset += push_bytes.len() / 2;
                    tracing::info!(target: "codegen", "OFFSET: {}, PUSH BYTES: {:?}", offset, push_bytes);
                    final_bytes.push(Bytes(push_bytes))
                }
                IRByte::Statement(s) => {
                    match s {
                        Statement::MacroInvocation(mi) => {
                            // Get the macro that matches this invocation and turn into bytecode
                            let ir_macro =
                                if let Some(m) = contract.find_macro_by_name(&mi.macro_name) {
                                    m
                                } else {
                                    // TODO: this is where the file imports must be resolved .. in
                                    // case macro definition is external
                                    tracing::error!(
                                        target: "codegen",
                                        "MISSING MACRO INVOCATION \"{}\"",
                                        mi.macro_name
                                    );
                                    return Err(CodegenError {
                                        kind: CodegenErrorKind::MissingMacroDefinition(
                                            mi.macro_name.clone(),
                                        ),
                                        span: None,
                                        token: None,
                                    })
                                };

                            tracing::info!(target: "codegen", "FOUND INNER MACRO: {:?}", ir_macro);

                            // Recurse
                            scope.push(ir_macro.clone());
                            let res: BytecodeRes = if let Ok(res) = Codegen::recurse_bytecode(
                                ir_macro.clone(),
                                ast.clone(),
                                scope,
                                offset,
                                jump_tables.clone(),
                            ) {
                                res
                            } else {
                                tracing::error!(
                                    target: "codegen",
                                    "FAILED TO RECURSE INTO MACRO \"{}\"",
                                    ir_macro.name
                                );
                                return Err(CodegenError {
                                    kind: CodegenErrorKind::FailedMacroRecursion,
                                    span: None,
                                    token: None,
                                })
                            };

                            // Set jump table values
                            jump_table.insert(index, res.unmatched_jumps);
                            jump_indices = jump_indices
                                .into_iter()
                                .chain(res.jump_indices)
                                .collect::<JumpIndices>();

                            // // Increase offset by byte length of recursed macro
                            offset +=
                                res.bytes.iter().map(|b| b.0.clone()).collect::<String>().len() / 2;

                            final_bytes = final_bytes
                                .iter()
                                .cloned()
                                .chain(res.bytes.iter().cloned())
                                .collect();
                        }
                        s => {
                            tracing::error!(target: "codegen", "UNEXPECTED STATEMENT: {:?}", s);
                            return Err(CodegenError {
                                kind: CodegenErrorKind::InvalidMacroStatement,
                                span: None,
                                token: None,
                            })
                        }
                    }
                }
                IRByte::ArgCall(arg_name) => {
                    // Try to find macro with same name as arg_name
                    let macro_with_arg_name =
                        if let Some(m) = contract.find_macro_by_name(&arg_name) {
                            m
                        } else {
                            tracing::error!("Invoked Macro \"{}\" not found in Contract", arg_name);
                            return Err(CodegenError {
                                kind: CodegenErrorKind::MissingMacroDefinition(arg_name.clone()),
                                span: None,
                                token: None,
                            })
                        };

                    // Lower scope and recurse into the found macro
                    scope.push(macro_with_arg_name.clone());
                    // TODO: Add proper parameters to the found macro definition
                    // https://github.com/huff-language/huffc/blob/master/src/compiler/processor.ts#L91-L98
                    let res = Codegen::recurse_bytecode(
                        macro_with_arg_name.clone(),
                        ast.clone(),
                        scope,
                        offset,
                        jump_tables.clone(),
                    );

                    if let Ok(res) = res {
                        // Set jump table values
                        jump_table.insert(index, res.unmatched_jumps);

                        // Increase offset by byte length of recursed macro
                        offset +=
                            res.bytes.iter().map(|b| b.0.clone()).collect::<String>().len() / 2;

                        // Add bytecode from arg call macro
                        final_bytes =
                            final_bytes.iter().cloned().chain(res.bytes.iter().cloned()).collect();
                    } else {
                        tracing::error!(
                            "Codegen failed to recurse into macro {}",
                            macro_with_arg_name.name
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::FailedMacroRecursion,
                            span: None,
                            token: None,
                        })
                    }

                    tracing::info!(target: "codegen", "FOUND ARG CALL TO \"{}\"", arg_name);
                }
            }
        }

        let mut cur_index = offset;
        let mut indices = vec![cur_index]; // first index is the current offset
        indices.append(
            &mut final_bytes
                .iter()
                .map(|b| {
                    cur_index += b.0.len() / 2;
                    cur_index
                })
                .collect::<Vec<usize>>(),
        );

        let mut unmatched_jumps = Jumps::default();
        let final_bytes =
            final_bytes.iter().enumerate().fold(Vec::default(), |mut acc, (index, b)| {
                let mut formatted_bytes = b.clone();

                if let Some(jt) = jump_table.get(&index) {
                    for jump in jt {
                        if let Some(jump_index) = jump_indices.get(jump) {
                            let jump_value = pad_n_bytes(&hex::encode(jump_index.to_string()), 2);

                            println!("Jump value: {}", jump_value);

                            let before = &formatted_bytes.0[0..jump.bytecode_index + 2];
                            let after = &formatted_bytes.0[jump.bytecode_index + 6..];

                            println!("Pre-formatted Bytes: {:?}", formatted_bytes);
                            formatted_bytes = Bytes(format!("{}{}{}", before, jump_value, after));
                            println!("Formatted Bytes: {:?}", formatted_bytes);
                        } else {
                            let jump_offset = (indices[index] - offset) * 2;

                            println!("Jump offset: {}", jump_offset);

                            unmatched_jumps.push(Jump {
                                label: jump.label.clone(),
                                bytecode_index: jump_offset + jump.bytecode_index,
                            })
                        }
                    }
                }

                acc.push(formatted_bytes);
                acc
            });

        Ok(BytecodeRes { bytes: final_bytes, jump_tables, jump_indices, unmatched_jumps })
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
        args: Vec<ethers::abi::token::Token>,
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

        let contract_size = format!("{:04x}", contract_length);
        let contract_code_offset = format!("{:04x}", 13 + constructor_length);

        let encoded: Vec<Vec<u8>> =
            args.iter().map(|tok| ethers::abi::encode(&[tok.clone()])).collect();
        let hex_args: Vec<String> = encoded.iter().map(|tok| hex::encode(tok.as_slice())).collect();
        let constructor_args = hex_args.join("");

        // Generate the final bytecode
        let bootstrap_code = format!("61{}8061{}6000396000f3", contract_size, contract_code_offset);
        let constructor_code = format!("{}{}", constructor_bytecode, bootstrap_code);
        artifact.bytecode = format!("{}{}{}", constructor_code, main_bytecode, constructor_args);
        artifact.runtime = main_bytecode.to_string();
        Ok(artifact.clone())
    }

    /// Encode constructor arguments as ethers::abi::token::Token
    pub fn encode_constructor_args(args: Vec<String>) -> Vec<ethers::abi::token::Token> {
        let tokens: Vec<ethers::abi::token::Token> =
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
    pub fn export(&self, output: String) -> Result<(), CodegenError> {
        if let Some(art) = &self.artifact {
            let serialized_artifact = serde_json::to_string(art).unwrap();
            fs::write(output, serialized_artifact).expect("Unable to write file");
        } else {
            tracing::error!(
                target: "codegen",
                "Failed to export the compile artifact to the specified output location {}!",
                output
            );
            return Err(CodegenError {
                kind: CodegenErrorKind::AbiGenerationFailure,
                span: None,
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
        match &mut self.artifact {
            Some(artifact) => {
                artifact.abi = Some(abi.clone());
            }
            None => {
                self.artifact = Some(Artifact { abi: Some(abi.clone()), ..Default::default() });
            }
        }

        // If an output's specified, write the artifact out
        if let Some(o) = output {
            if let Err(e) = self.export(o) {
                // Error message is sent to tracing in `export` if an error occurs
                return Err(e)
            }
        }

        // Return the abi
        Ok(abi)
    }
}
