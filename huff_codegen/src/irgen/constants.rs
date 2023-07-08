use huff_utils::prelude::{
    literal_gen, AstSpan, CodegenError, CodegenErrorKind, ConstVal, Contract, EVMVersion,
};

/// Transforms a constant definition into it's respective bytecode
pub fn constant_gen(
    evm_version: &EVMVersion,
    name: &str,
    contract: &Contract,
    ir_byte_span: &AstSpan,
) -> Result<String, CodegenError> {
    // Get the first `ConstantDefinition` that matches the constant's name
    let constants = contract
        .constants
        .lock()
        .map_err(|_| CodegenError::new(CodegenErrorKind::LockingError, AstSpan(vec![]), None))?;
    let constant = if let Some(m) = constants.iter().find(|const_def| const_def.name.eq(&name)) {
        m
    } else {
        tracing::error!(target: "codegen", "MISSING CONSTANT DEFINITION \"{}\"", name);

        return Err(CodegenError {
            kind: CodegenErrorKind::MissingConstantDefinition(name.to_string()),
            span: ir_byte_span.clone(),
            token: None,
        })
    };

    // Generate bytecode for the constant
    // Should always be a `Literal` if storage pointers were derived in the AST
    // prior to generating the IR bytes.
    tracing::info!(target: "codegen", "FOUND CONSTANT DEFINITION: {}", constant.name);
    let push_bytes = match &constant.value {
        ConstVal::Literal(l) => literal_gen(evm_version, l),
        ConstVal::FreeStoragePointer(fsp) => {
            // If this is reached in codegen stage, the `derive_storage_pointers`
            // method was not called on the AST.
            tracing::error!(target: "codegen", "STORAGE POINTERS INCORRECTLY DERIVED FOR \"{:?}\"", fsp);
            return Err(CodegenError {
                kind: CodegenErrorKind::StoragePointersNotDerived,
                span: constant.span.clone(),
                token: None,
            })
        }
    };

    Ok(push_bytes)
}
