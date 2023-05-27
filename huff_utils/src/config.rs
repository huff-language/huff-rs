use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use crate::{evm_version::EVMVersion, foundry::FoundryConfig, remapper::Remapper};

/// Stores external configuration options, such as remappings and evm version.

#[derive(Debug, Default, Clone)]
pub struct HuffConfig {
    pub base_dir: String,
    pub evm_version: EVMVersion,
    pub remappings: HashMap<String, String>,
}

impl HuffConfig {
    pub fn new(root: impl AsRef<str>) -> Self {
        let base_dir = root.as_ref().to_string();

        // Parse foundry config and remappings
        let foundry_config = FoundryConfig::new(&base_dir);
        let file_remappings = remappings_from_file(&base_dir);

        let mut remappings = HashMap::<String, String>::new();
        remappings.extend(file_remappings);
        remappings.extend(foundry_config.remappings);

        let evm_version = EVMVersion::from(foundry_config.evm_version);

        HuffConfig { base_dir, evm_version, remappings }
    }

    pub fn from_evm_version(evm_version: EVMVersion) -> Self {
        HuffConfig { base_dir: "".to_string(), evm_version, remappings: HashMap::new() }
    }
}

impl Remapper for HuffConfig {
    /// Tries to replace path segments in a string with our remappings
    fn remap(&self, path: &str) -> Option<String> {
        let mut path = path.to_string();
        for (k, v) in self.remappings.iter() {
            if path.starts_with(k) {
                tracing::debug!(target: "parser", "found key {} and value {}", k, v);
                path = path.replace(k, v);
                return Some(format!("{}{path}", self.base_dir))
            }
        }
        None
    }
}

// Read remappings from remappings.txt
fn remappings_from_file(root: &str) -> HashMap<String, String> {
    let mut remappings: HashMap<String, String> = HashMap::new();
    let remappings_file = PathBuf::new().join(root).join("remappings.txt");
    if remappings_file.is_file() {
        let content = read_to_string(remappings_file).map_err(|err| err.to_string()).unwrap();

        let rem_lines = content.split('\n').collect::<Vec<&str>>();
        let rem = rem_lines
            .iter()
            .filter(|l| l != &&"")
            .map(|l| l.split_once('='))
            .collect::<Vec<Option<(&str, &str)>>>();
        rem.iter().for_each(|pair| {
            if let Some((lib, path)) = pair {
                remappings.insert(lib.to_string(), path.to_string());
            }
        });
    }
    remappings
}
