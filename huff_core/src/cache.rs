use std::sync::Arc;

use huff_utils::prelude::{Artifact, FileSource, OutputLocation};
use walkdir::WalkDir;

/// Parallelized Artifact Cachcing
pub fn get_cached_artifacts(
    files: &[Arc<FileSource>],
    out: &OutputLocation,
    constructor_args: String,
) -> Option<Vec<Arc<Artifact>>> {
    // Check if the file artifacts are already generated the the default "./artifacts/" directory or
    // the specified output dir
    let artifacts: Vec<(Arc<FileSource>, Artifact)> =
        resolve_existing_artifacts(files, out, constructor_args)?;

    // Return the artifacts if cached
    Some(artifacts.into_iter().map(|(_, artifact)| Arc::new(artifact)).collect())
}

/// Attempt to grab the artifacts
pub fn resolve_existing_artifacts(
    files: &[Arc<FileSource>],
    output: &OutputLocation,
    constructor_args: String,
) -> Option<Vec<(Arc<FileSource>, Artifact)>> {
    let mut artifacts: Vec<(Arc<FileSource>, Artifact)> = Vec::new();

    // Transform file sources into a hashmap of path to file source
    let mut file_sources: std::collections::HashMap<String, Arc<FileSource>> =
        files.iter().map(|f| (f.path.clone().to_lowercase(), Arc::clone(f))).collect();

    // If outputdir is not specified, use the default "./artifacts/" directory
    let output_dir = if !output.0.is_empty() { &*output.0 } else { "./artifacts" };

    // For each file, check if the artifact file exists at the location
    tracing::debug!(target: "core", "Traversing output directory {}", output_dir);
    for entry in WalkDir::new(output_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        // Are we expecting this file to be compiled
        let formatted_path = entry
            .path()
            .display()
            .to_string()
            .replace(".json", "")
            .replace(output_dir, ".")
            .to_lowercase();
        let expected = file_sources.remove(&formatted_path);

        // Try to read the file into an artifact
        match serde_json::from_str::<Artifact>(&std::fs::read_to_string(entry.path()).unwrap()) {
            Ok(artifact) => {
                // If we expected compilation, the sources must match
                match expected {
                    Some(expected_fs) => {
                        if !artifact.bytecode.ends_with(&constructor_args) {
                            tracing::warn!(target: "core", "Mismatched Constructor Args for Cached Artifact \"{}\"", artifact.file.path);
                            return None
                        }
                        if artifact.file.source != expected_fs.source {
                            tracing::warn!(target: "core", "Cache Resolution Failed: \"{}\" Artifact Outdated", artifact.file.path);
                            return None
                        } else {
                            artifacts.push((expected_fs, artifact));
                        }
                    }
                    None => {
                        tracing::warn!(target: "core", "Cache Resolution Found Unexpected Artifact: \"{}\"", artifact.file.path)
                    }
                }
            }
            Err(e) => {
                // If the artifact is invalid, log the error and continue
                tracing::error!(target: "core", "Invalid artifact file: {}", e);
                if expected.is_some() {
                    tracing::error!(target: "core", "Expected artifact file to be compiled: {}", entry.path().display());
                    return None
                }
            }
        }
    }

    match file_sources.is_empty() {
        true => Some(artifacts),
        false => {
            tracing::warn!(target: "core", "Cache Resolution Failed: Missing Artifact Files");
            None
        }
    }
}
