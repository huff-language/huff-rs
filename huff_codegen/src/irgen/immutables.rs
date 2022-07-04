use huff_utils::prelude::{
    bytes32_to_string, AstSpan, CodegenError, CodegenErrorKind, ConstVal, Contract,
};

/// Transforms an immutable definition into it's respective bytecode
pub fn immutable_gen(
    name: &str,
    contract: &Contract,
    ir_byte_span: AstSpan,
) -> Result<String, CodegenError> {
    // Get the first `ImmutableDefinition` that matches the immutable's name
    let immutable = if let Some(m) = contract.immutables.iter().find(|idef| idef.name.eq(&name)) {
        m
    } else {
        tracing::error!(target: "codegen", "MISSING IMMUTABLE DEFINITION \"{}\"", name);

        return Err(CodegenError {
            kind: CodegenErrorKind::MissingImmutableDefinition(name.to_string()),
            span: ir_byte_span,
            token: None,
        })
    };

    // Generate bytecode for the immutable
    // Should always be a `Literal` if storage pointers were derived in the AST
    // prior to generating the IR bytes.
    tracing::info!(target: "codegen", "FOUND IMMUTABLE DEFINITION: {}", immutable.name);
    let push_bytes = match &immutable.value {
        Some(ConstVal::Literal(l)) => {
            let hex_literal: String = bytes32_to_string(l, false);
            format!("{:02x}{}", 95 + hex_literal.len() / 2, hex_literal)
        }
        Some(ConstVal::FreeStoragePointer(fsp)) => {
            // If this is reached in codegen stage, the `derive_storage_pointers`
            // method was not called on the AST.
            tracing::error!(target: "codegen", "STORAGE POINTERS INCORRECTLY DERIVED FOR \"{:?}\"", fsp);
            return Err(CodegenError {
                kind: CodegenErrorKind::StoragePointersNotDerived,
                span: immutable.span.clone(),
                token: None,
            })
        }
        None => {
            tracing::error!(target: "codegen", "MISSING VALUE FOR IMMUTABLE \"{}\"", immutable.name);
            return Err(CodegenError {
                kind: CodegenErrorKind::StoragePointersNotDerived,
                span: immutable.span.clone(),
                token: None,
            })
        }
    };

    Ok(push_bytes)
}
