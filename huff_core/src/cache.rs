use huff_utils::prelude::{OutputLocation, FileSource, Artifact};


/// Parallelized Artifact Cachcing
pub fn get_cached_artifacts(files: Vec<Arc<FileSource>>, out: OutputLocation) -> Option<Vec<Artifact>> {
  // Check if the file artifacts are already generated the the default "./artifacts/" directory or the specified output dir
  let artifacts: Vec<(Arc<FileSource>, Artifact)> = resolve_existing_artifacts(out);

  // For each 
  

}

/// Attempt to grab the artifacts
pub fn resolve_existing_artifacts(output: OutputLocation) -> Vec<(Arc<FileSource>, Artifact)>  {
  // TODO: for each file, check if the artifact file exists at the location
  let json_out = format!("{}/{}.json", output.0, a.file.path.to_uppercase().replacen("./", "", 1));

  // TODO: Construct Artifact from the FileSource

  // TODO: return tuple of file source -> Artifact reference
}