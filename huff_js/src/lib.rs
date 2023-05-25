#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use std::{collections::HashMap, sync::Arc};

use wasm_bindgen::prelude::*;

use huff_core::Compiler;
use huff_utils::{abi::Abi, artifact::Artifact, error::CompilerError, prelude::EVMVersion};
use serde::{Deserialize, Serialize};

/// Converts a CompilerError into a returnable JsValue
fn compiler_error_to_js_value(ce: Arc<CompilerError>) -> JsValue {
    let output = CompilerOutput { errors: Some(vec![format!("{}", *ce)]), contracts: None };
    serde_wasm_bindgen::to_value(&output).unwrap_or(JsValue::NULL)
}

#[derive(Serialize, Deserialize)]
struct CompilerInput {
    evm_version: Option<String>,
    sources: Vec<String>,
    files: HashMap<String, String>,
    construct_args: Option<Vec<String>>,
    alternative_main: Option<String>,
    alternative_constructor: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CompilerArtifact {
    bytecode: String,
    runtime: String,
    abi: Option<Abi>,
}

#[derive(Serialize, Deserialize)]
struct CompilerOutput {
    errors: Option<Vec<String>>,
    contracts: Option<HashMap<String, CompilerArtifact>>,
}

/// Compiles contracts based on supplied JSON input
#[wasm_bindgen]
pub fn compile(input: JsValue) -> Result<JsValue, JsValue> {
    let input: CompilerInput = serde_wasm_bindgen::from_value(input)?;

    let evm_version = EVMVersion::from(input.evm_version);

    let compiler = Compiler::new_in_memory(
        &evm_version,
        Arc::new(input.sources),
        input.files,
        input.alternative_main,
        input.alternative_constructor,
        input.construct_args,
        None,
        false,
    );
    let res: Vec<Arc<Artifact>> = compiler.execute().map_err(compiler_error_to_js_value)?;

    let mut contracts: HashMap<String, CompilerArtifact> = HashMap::new();

    res.into_iter().for_each(|artifact| {
        contracts.insert(
            artifact.file.path.clone(),
            CompilerArtifact {
                bytecode: artifact.bytecode.clone(),
                runtime: artifact.runtime.clone(),
                abi: artifact.abi.clone(),
            },
        );
    });

    let output = CompilerOutput { errors: None, contracts: Some(contracts) };

    serde_wasm_bindgen::to_value(&output).map_err(|_| JsValue::NULL)
}
