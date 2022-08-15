use huff_utils::prelude::*;

use crate::Codegen;

/// Generates the respective Bytecode for a given Statement
#[allow(clippy::too_many_arguments)]
pub fn statement_gen(
    s: &Statement,
    contract: &Contract,
    macro_def: &MacroDefinition,
    scope: &mut Vec<MacroDefinition>,
    offset: &mut usize,
    mis: &mut Vec<(usize, MacroInvocation)>,
    jump_table: &mut JumpTable,
    label_indices: &mut LabelIndices,
    table_instances: &mut Jumps,
    utilized_tables: &mut Vec<TableDefinition>,
    starting_offset: usize,
) -> Result<Vec<(usize, Bytes)>, CodegenError> {
    let mut bytes = vec![];

    tracing::debug!(target: "codegen", "Got Statement: {}", s.ty);

    match &s.ty {
        StatementType::MacroInvocation(mi) => {
            // Get the macro definition that matches the name of this invocation
            let ir_macro = if let Some(m) = contract.find_macro_by_name(&mi.macro_name) {
                m
            } else {
                tracing::error!(
                    target: "codegen",
                    "MISSING MACRO INVOCATION \"{}\"",
                    mi.macro_name
                );
                return Err(CodegenError {
                    kind: CodegenErrorKind::InvalidMacroInvocation(mi.macro_name.clone()),
                    span: mi.span.clone(),
                    token: None,
                })
            };

            tracing::info!(target: "codegen", "FOUND INNER MACRO: {}", ir_macro.name);

            // Tests may not be invoked
            if ir_macro.test {
                tracing::error!(target: "codegen", "Tests may not be invoked: {}", ir_macro.name);
                return Err(CodegenError {
                    kind: CodegenErrorKind::TestInvocation(ir_macro.name.clone()),
                    span: ir_macro.span,
                    token: None,
                })
            }

            // If invoked macro is a function (outlined), insert a jump to the function's code and a
            // jumpdest to return to. If it is inlined, insert the macro's code at the
            // current offset.
            if ir_macro.outlined {
                // Get necessary swap ops to reorder stack
                // PC of the return jumpdest should be below the function's stack inputs
                let stack_swaps = (0..ir_macro.takes)
                    .rev()
                    .map(|i| format!("{:02x}", 0x90 + i))
                    .collect::<Vec<_>>();

                // Insert a jump to the outlined macro's code
                jump_table.insert(
                    *offset + stack_swaps.len() + 3, // PUSH2 + 2 bytes + stack_swaps.len()
                    vec![Jump {
                        label: format!("goto_{}", &ir_macro.name),
                        bytecode_index: 0,
                        span: s.span.clone(),
                    }],
                );

                // Store return JUMPDEST PC on the stack and re-order the stack so that
                // the return JUMPDEST PC is below the function's stack inputs
                bytes.push((
                    *offset,
                    Bytes(format!(
                        "{}{:04x}{}",
                        Opcode::Push2,
                        *offset + stack_swaps.len() + 7,
                        stack_swaps.join("")
                    )),
                ));
                // Insert jump to outlined macro + jumpdest to return to
                bytes.push((
                    *offset + stack_swaps.len() + 3, // PUSH2 + 2 bytes + stack_swaps.len()
                    Bytes(format!("{}xxxx{}{}", Opcode::Push2, Opcode::Jump, Opcode::Jumpdest)),
                ));
                // PUSH2 + 2 bytes + stack_swaps.len() + PUSH2 + 2 bytes + JUMP + JUMPDEST
                *offset += stack_swaps.len() + 8;
            } else {
                // Recurse into macro invocation
                scope.push(ir_macro.clone());
                mis.push((*offset, mi.clone()));

                let mut res: BytecodeRes = match Codegen::macro_to_bytecode(
                    ir_macro.clone(),
                    contract,
                    scope,
                    *offset,
                    mis,
                ) {
                    Ok(r) => r,
                    Err(e) => {
                        tracing::error!(
                            target: "codegen",
                            "FAILED TO RECURSE INTO MACRO \"{}\"",
                            ir_macro.name
                        );
                        return Err(e)
                    }
                };

                // Set jump table values
                tracing::debug!(target: "codegen", "Unmatched jumps: {:?}", res.unmatched_jumps.iter().map(|uj| uj.label.clone()).collect::<Vec<String>>());
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
                utilized_tables.extend(res.utilized_tables);

                // Increase offset by byte length of recursed macro
                *offset += res.bytes.iter().map(|(_, b)| b.0.len()).sum::<usize>() / 2;
                // Add the macro's bytecode to the final result
                bytes = [bytes, res.bytes].concat()
            }
        }
        StatementType::Label(label) => {
            // Add JUMPDEST opcode to final result and add to label_indices
            tracing::info!(target: "codegen", "RECURSE BYTECODE GOT LABEL: {:?}", label.name);
            label_indices.insert(label.name.clone(), *offset);
            bytes.push((*offset, Bytes(Opcode::Jumpdest.to_string())));
            *offset += 1;
        }
        StatementType::LabelCall(label) => {
            // Generate code for a `LabelCall`
            // PUSH2 + 2 byte destination (placeholder for now, filled in `Codegen::fill_unmatched`
            tracing::info!(target: "codegen", "RECURSE BYTECODE GOT LABEL CALL: {}", label);
            jump_table.insert(
                *offset,
                vec![Jump { label: label.to_string(), bytecode_index: 0, span: s.span.clone() }],
            );
            bytes.push((*offset, Bytes(format!("{}xxxx", Opcode::Push2))));
            *offset += 3;
        }
        StatementType::BuiltinFunctionCall(bf) => {
            // Generate code for a `BuiltinFunctionCall`
            // __codesize, __tablesize, or __tablestart
            tracing::info!(target: "codegen", "RECURSE BYTECODE GOT BUILTIN FUNCTION CALL: {:?}", bf);
            match bf.kind {
                BuiltinFunctionKind::Codesize => {
                    let ir_macro = if let Some(m) =
                        contract.find_macro_by_name(bf.args[0].name.as_ref().unwrap())
                    {
                        m
                    } else {
                        tracing::error!(
                            target: "codegen",
                            "MISSING MACRO PASSED TO __codesize \"{}\"",
                            bf.args[0].name.as_ref().unwrap()
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::MissingMacroDefinition(
                                bf.args[0].name.as_ref().unwrap().to_string(), /* yuck */
                            ),
                            span: bf.span.clone(),
                            token: None,
                        })
                    };

                    let res: BytecodeRes = match Codegen::macro_to_bytecode(
                        ir_macro.clone(),
                        contract,
                        scope,
                        *offset,
                        mis,
                    ) {
                        Ok(r) => r,
                        Err(e) => {
                            tracing::error!(
                                target: "codegen",
                                "FAILED TO RECURSE INTO MACRO \"{}\"",
                                ir_macro.name
                            );
                            return Err(e)
                        }
                    };

                    let size = format_even_bytes(format!(
                        "{:02x}",
                        (res.bytes.iter().map(|(_, b)| b.0.len()).sum::<usize>() / 2)
                    ));
                    let push_bytes = format!("{:02x}{}", 95 + size.len() / 2, size);

                    *offset += push_bytes.len() / 2;
                    bytes.push((starting_offset, Bytes(push_bytes)));
                }
                BuiltinFunctionKind::Tablesize => {
                    let ir_table = if let Some(t) =
                        contract.find_table_by_name(bf.args[0].name.as_ref().unwrap())
                    {
                        t
                    } else {
                        tracing::error!(
                            target: "codegen",
                            "MISSING TABLE PASSED TO __tablesize \"{}\"",
                            bf.args[0].name.as_ref().unwrap()
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::InvalidMacroInvocation(
                                bf.args[0].name.as_ref().unwrap().to_string(), /* yuck */
                            ),
                            span: bf.span.clone(),
                            token: None,
                        })
                    };

                    let size = bytes32_to_string(&ir_table.size, false);
                    let push_bytes = format!("{:02x}{}", 95 + size.len() / 2, size);

                    if !utilized_tables.contains(&ir_table) {
                        utilized_tables.push(ir_table);
                    }

                    *offset += push_bytes.len() / 2;
                    bytes.push((starting_offset, Bytes(push_bytes)));
                }
                BuiltinFunctionKind::Tablestart => {
                    // Make sure the table exists
                    if let Some(t) = contract.find_table_by_name(bf.args[0].name.as_ref().unwrap())
                    {
                        table_instances.push(Jump {
                            label: bf.args[0].name.as_ref().unwrap().to_owned(),
                            bytecode_index: *offset,
                            span: bf.span.clone(),
                        });
                        if !utilized_tables.contains(&t) {
                            utilized_tables.push(t);
                        }

                        bytes.push((*offset, Bytes(format!("{}xxxx", Opcode::Push2))));
                        *offset += 3;
                    } else {
                        tracing::error!(
                            target: "codegen",
                            "MISSING TABLE PASSED TO __tablestart \"{}\"",
                            bf.args[0].name.as_ref().unwrap()
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::InvalidMacroInvocation(
                                bf.args[0].name.as_ref().unwrap().to_string(),
                            ),
                            span: bf.span.clone(),
                            token: None,
                        })
                    }
                }
                BuiltinFunctionKind::FunctionSignature => {
                    if bf.args.len() != 1 {
                        tracing::error!(
                            target: "codegen",
                            "Incorrect number of arguments passed to __FUNC_SIG, should be 1: {}",
                            bf.args.len()
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::InvalidArguments(
                                format!(
                                    "Incorrect number of arguments passed to __FUNC_SIG, should be 1: {}",
                                    bf.args.len()
                                )
                            ),
                            span: bf.span.clone(),
                            token: None,
                        })
                    }

                    if let Some(func) = contract
                        .functions
                        .iter()
                        .find(|f| bf.args[0].name.as_ref().unwrap().eq(&f.name))
                    {
                        let push_bytes =
                            format!("{}{}", Opcode::Push4, hex::encode(func.signature));
                        *offset += push_bytes.len() / 2;
                        bytes.push((starting_offset, Bytes(push_bytes)));
                    } else if let Some(s) = &bf.args[0].name {
                        let mut signature = [0u8; 4]; // Only keep first 4 bytes
                        hash_bytes(&mut signature, s);

                        let push_bytes = format!("{}{}", Opcode::Push4, hex::encode(signature));
                        *offset += push_bytes.len() / 2;
                        bytes.push((starting_offset, Bytes(push_bytes)));
                    } else {
                        tracing::error!(
                            target: "codegen",
                            "MISSING FUNCTION INTERFACE PASSED TO __SIG: \"{}\"",
                            bf.args[0].name.as_ref().unwrap()
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::MissingFunctionInterface(
                                bf.args[0].name.as_ref().unwrap().to_string(),
                            ),
                            span: bf.span.clone(),
                            token: None,
                        })
                    }
                }
                BuiltinFunctionKind::EventHash => {
                    if bf.args.len() != 1 {
                        tracing::error!(
                            target: "codegen",
                            "Incorrect number of arguments passed to __EVENT_HASH, should be 1: {}",
                            bf.args.len()
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::InvalidArguments(
                                format!(
                                    "Incorrect number of arguments passed to __EVENT_HASH, should be 1: {}",
                                    bf.args.len()
                                )
                            ),
                            span: bf.span.clone(),
                            token: None,
                        })
                    }

                    if let Some(event) = contract
                        .events
                        .iter()
                        .find(|e| bf.args[0].name.as_ref().unwrap().eq(&e.name))
                    {
                        let hash = bytes32_to_string(&event.hash, false);
                        let push_bytes = format!("{}{}", Opcode::Push32, hash);
                        *offset += push_bytes.len() / 2;
                        bytes.push((starting_offset, Bytes(push_bytes)));
                    } else if let Some(s) = &bf.args[0].name {
                        let mut hash = [0u8; 32];
                        hash_bytes(&mut hash, s);

                        let push_bytes = format!("{}{}", Opcode::Push32, hex::encode(hash));
                        *offset += push_bytes.len() / 2;
                        bytes.push((starting_offset, Bytes(push_bytes)));
                    } else {
                        tracing::error!(
                            target: "codegen",
                            "MISSING EVENT INTERFACE PASSED TO __EVENT_HASH: \"{}\"",
                            bf.args[0].name.as_ref().unwrap()
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::MissingEventInterface(
                                bf.args[0].name.as_ref().unwrap().to_string(),
                            ),
                            span: bf.span.clone(),
                            token: None,
                        })
                    }
                }
                BuiltinFunctionKind::Error => {
                    if bf.args.len() != 1 {
                        tracing::error!(
                            target: "codegen",
                            "Incorrect number of arguments passed to __ERROR, should be 1: {}",
                            bf.args.len()
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::InvalidArguments(format!(
                                "Incorrect number of arguments passed to __ERROR, should be 1: {}",
                                bf.args.len()
                            )),
                            span: bf.span.clone(),
                            token: None,
                        })
                    }

                    if let Some(error) = contract
                        .errors
                        .iter()
                        .find(|e| bf.args[0].name.as_ref().unwrap().eq(&e.name))
                    {
                        // Add 28 bytes to left-pad the 4 byte selector
                        let selector =
                            format!("{}{}", hex::encode(error.selector), "00".repeat(28));
                        let push_bytes = format!("{}{}", Opcode::Push32, selector);
                        *offset += push_bytes.len() / 2;
                        bytes.push((starting_offset, Bytes(push_bytes)));
                    } else {
                        tracing::error!(
                            target: "codegen",
                            "MISSING ERROR DEFINITION PASSED TO __ERROR: \"{}\"",
                            bf.args[0].name.as_ref().unwrap()
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::MissingErrorDefinition(
                                bf.args[0].name.as_ref().unwrap().to_string(),
                            ),
                            span: bf.span.clone(),
                            token: None,
                        })
                    }
                }
                BuiltinFunctionKind::RightPad => {
                    if bf.args.len() != 1 {
                        tracing::error!(
                            target = "codegen",
                            "Incorrect number of arguments passed to __RIGHTPAD, should be 1: {}",
                            bf.args.len()
                        );
                        return Err(CodegenError {
                            kind: CodegenErrorKind::InvalidArguments(format!(
                                "Incorrect number of arguments passed to __RIGHTPAD, should be 1: {}",
                                bf.args.len()
                            )),
                            span: bf.span.clone(),
                            token: None,
                        })
                    }

                    let hex = format_even_bytes(bf.args[0].name.as_ref().unwrap().clone());
                    let push_bytes =
                        format!("{}{}{}", Opcode::Push32, hex, "0".repeat(64 - hex.len()));
                    *offset += push_bytes.len() / 2;
                    bytes.push((starting_offset, Bytes(push_bytes)));
                }
            }
        }
        sty => {
            tracing::error!(target: "codegen", "CURRENT MACRO DEF: {}", macro_def.name);
            tracing::error!(target: "codegen", "UNEXPECTED STATEMENT: {:?}", sty);
            return Err(CodegenError {
                kind: CodegenErrorKind::InvalidMacroStatement,
                span: s.span.clone(),
                token: None,
            })
        }
    }

    Ok(bytes)
}
