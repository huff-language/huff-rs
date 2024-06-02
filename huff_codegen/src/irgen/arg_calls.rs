use huff_utils::prelude::*;
use std::str::FromStr;

use crate::Codegen;

// Arguments can be literals, labels, opcodes, or constants
// !! IF THERE IS AMBIGUOUS NOMENCLATURE
// !! (E.G. BOTH OPCODE AND LABEL ARE THE SAME STRING)
// !! COMPILATION _WILL_ ERROR

/// Arg Call Bubbling
#[allow(clippy::too_many_arguments)]
pub fn bubble_arg_call(
    evm_version: &EVMVersion,
    arg_name: &str,
    bytes: &mut Vec<(usize, Bytes)>,
    macro_def: &MacroDefinition,
    contract: &Contract,
    scope: &mut [&MacroDefinition],
    offset: &mut usize,
    // mis: Parent macro invocations and their indices
    mis: &mut [(usize, MacroInvocation)],
    jump_table: &mut JumpTable,
    circular_codesize_invocations: &mut CircularCodeSizeIndices,
    label_indices: &mut LabelIndices,
    table_instances: &mut Jumps,
) -> Result<(), CodegenError> {
    let starting_offset = *offset;

    if let Some(macro_invoc) = mis.last() {
        // Literal, Ident & Arg Call Check
        // First get this arg_nam position in the macro definition params
        if let Some(pos) = macro_def
            .parameters
            .iter()
            .position(|r| r.name.as_ref().map_or(false, |s| s.eq(arg_name)))
        {
            tracing::info!(target: "codegen", "GOT \"{}\" POS IN ARG LIST: {}", arg_name, pos);

            if let Some(arg) = macro_invoc.1.args.get(pos) {
                tracing::info!(target: "codegen", "GOT \"{:?}\" ARG FROM MACRO INVOCATION", arg);
                match arg {
                    MacroArg::Literal(l) => {
                        tracing::info!(target: "codegen", "GOT LITERAL {} ARG FROM MACRO INVOCATION", bytes32_to_string(l, false));

                        let hex_literal: String = bytes32_to_string(l, false);
                        let push_bytes = format!("{:02x}{hex_literal}", 95 + hex_literal.len() / 2);
                        let b = Bytes(push_bytes);
                        *offset += b.0.len() / 2;
                        bytes.push((starting_offset, b));
                    }
                    MacroArg::ArgCall(ac) => {
                        tracing::info!(target: "codegen", "GOT ARG CALL \"{}\" ARG FROM MACRO INVOCATION", ac);
                        tracing::debug!(target: "codegen", "~~~ BUBBLING UP ARG CALL");
                        let scope_len = scope.len();
                        let new_scope = &mut scope[..scope_len.saturating_sub(1)];
                        let bubbled_macro_invocation = new_scope.last().unwrap();
                        tracing::debug!(target: "codegen", "BUBBLING UP WITH MACRO DEF: {}", &bubbled_macro_invocation.name);
                        tracing::debug!(target: "codegen", "CURRENT MACRO DEF: {}", macro_def.name);

                        // Only remove an invocation if not at bottom level, otherwise we'll
                        // remove one too many
                        let last_mi = match mis.last() {
                            Some(mi) => mi,
                            None => {
                                return Err(CodegenError {
                                    kind: CodegenErrorKind::MissingMacroInvocation(
                                        macro_def.name.clone(),
                                    ),
                                    span: bubbled_macro_invocation.span.clone(),
                                    token: None,
                                })
                            }
                        };
                        let mis_len = mis.len();
                        let ac_ = &ac.to_string();
                        return if last_mi.1.macro_name.eq(&macro_def.name) {
                            bubble_arg_call(
                                evm_version,
                                ac_,
                                bytes,
                                bubbled_macro_invocation,
                                contract,
                                new_scope,
                                offset,
                                &mut mis[..mis_len.saturating_sub(1)],
                                jump_table,
                                circular_codesize_invocations,
                                label_indices,
                                table_instances,
                            )
                        } else {
                            bubble_arg_call(
                                evm_version,
                                ac_,
                                bytes,
                                bubbled_macro_invocation,
                                contract,
                                new_scope,
                                offset,
                                mis,
                                jump_table,
                                circular_codesize_invocations,
                                label_indices,
                                table_instances,
                            )
                        };
                    }
                    MacroArg::Ident(iden) => {
                        tracing::debug!(target: "codegen", "Found MacroArg::Ident IN \"{}\" Macro Invocation: \"{}\"!", macro_invoc.1.macro_name, iden);

                        // The opcode check needs to happens before the constants lookup
                        // because otherwise the mutex can deadlock when bubbling up to
                        // resolve macros as arguments.
                        if let Ok(o) = Opcode::from_str(iden) {
                            tracing::debug!(target: "codegen", "Found Opcode: {}", o);
                            let b = Bytes(o.to_string());
                            *offset += b.0.len() / 2;
                            bytes.push((starting_offset, b));
                        } else if let Some(constant) = contract
                            .constants
                            .lock()
                            .map_err(|_| {
                                CodegenError::new(
                                    CodegenErrorKind::LockingError,
                                    AstSpan(vec![]),
                                    None,
                                )
                            })?
                            .iter()
                            .find(|const_def| const_def.name.eq(iden))
                        {
                            tracing::info!(target: "codegen", "ARGCALL IS CONSTANT: {:?}", constant);
                            let push_bytes = match &constant.value {
                                ConstVal::Literal(l) => {
                                    let hex_literal: String = bytes32_to_string(l, false);
                                    format!("{:02x}{hex_literal}", 95 + hex_literal.len() / 2)
                                }
                                ConstVal::FreeStoragePointer(fsp) => {
                                    // If this is reached in codegen stage,
                                    // `derive_storage_pointers`
                                    // method was not called on the AST.
                                    tracing::error!(target: "codegen", "STORAGE POINTERS INCORRECTLY DERIVED FOR \"{:?}\"", fsp);
                                    return Err(CodegenError {
                                        kind: CodegenErrorKind::StoragePointersNotDerived,
                                        span: AstSpan(vec![]),
                                        token: None,
                                    });
                                }
                            };
                            *offset += push_bytes.len() / 2;
                            tracing::info!(target: "codegen", "OFFSET: {}, PUSH BYTES: {:?}", offset, push_bytes);
                            bytes.push((starting_offset, Bytes(push_bytes)));
                        } else if let Some(ir_macro) = contract.find_macro_by_name(iden) {
                            tracing::debug!(target: "codegen", "ARG CALL IS MACRO: {}", iden);
                            tracing::debug!(target: "codegen", "CURRENT MACRO DEF: {}", macro_def.name);

                            let mut new_scopes = scope.to_vec();
                            new_scopes.push(ir_macro);
                            let mut new_mis = mis.to_vec();
                            new_mis.push((
                                *offset,
                                MacroInvocation {
                                    macro_name: iden.to_string(),
                                    args: vec![],
                                    span: AstSpan(vec![]),
                                },
                            ));

                            let mut res: BytecodeRes = match Codegen::macro_to_bytecode(
                                evm_version,
                                ir_macro,
                                contract,
                                &mut new_scopes,
                                *offset,
                                &mut new_mis,
                                false,
                                Some(circular_codesize_invocations),
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

                            for j in res.unmatched_jumps.iter_mut() {
                                let new_index = j.bytecode_index;
                                j.bytecode_index = 0;
                                let mut new_jumps = if let Some(jumps) = jump_table.get(&new_index)
                                {
                                    jumps.clone()
                                } else {
                                    vec![]
                                };
                                new_jumps.push(j.clone());
                                jump_table.insert(new_index, new_jumps);
                            }
                            table_instances.extend(res.table_instances);
                            label_indices.extend(res.label_indices);

                            // Increase offset by byte length of recursed macro
                            *offset += res.bytes.iter().map(|(_, b)| b.0.len()).sum::<usize>() / 2;
                            // Add the macro's bytecode to the final result
                            res.bytes.iter().for_each(|(a, b)| bytes.push((*a, b.clone())));
                        } else {
                            tracing::debug!(target: "codegen", "Found Label Call: {}", iden);

                            // This should be equivalent to a label call.
                            bytes.push((*offset, Bytes(format!("{}xxxx", Opcode::Push2))));
                            jump_table.insert(
                                *offset,
                                vec![Jump {
                                    label: iden.to_owned(),
                                    bytecode_index: 0,
                                    span: macro_invoc.1.span.clone(),
                                }],
                            );
                            *offset += 3;
                        }
                    }
                }
            } else {
                tracing::warn!(target: "codegen", "\"{}\" FOUND IN MACRO DEF BUT NOT IN MACRO INVOCATION!", arg_name);
            }
        } else {
            tracing::warn!(target: "codegen", "\"{}\" NOT IN ARG LIST", arg_name);
        }
    } else {
        // This is a label call
        tracing::info!(target: "codegen", "RECURSE_BYTECODE ARG CALL DEFAULTING TO LABEL CALL: \"{}\"", arg_name);
        let new_span = match mis.last() {
            Some(mi) => mi.1.span.clone(),
            None => AstSpan(vec![]),
        };
        jump_table.insert(
            mis.last().map(|mi| mi.0).unwrap_or_else(|| 0),
            vec![Jump { label: arg_name.to_owned(), bytecode_index: 0, span: new_span }],
        );
        bytes.push((*offset, Bytes(format!("{}xxxx", Opcode::Push2))));
        *offset += 3;
    }

    Ok(())
}
