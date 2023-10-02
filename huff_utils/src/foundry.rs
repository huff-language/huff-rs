use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use itertools::Itertools;

/// Foundry.toml reading support
pub struct FoundryConfig {
    path: PathBuf,
    // The settings we care about
    pub remappings: HashMap<String, String>,
    // The evm version
    pub evm_version: Option<String>,
}

impl FoundryConfig {
    pub fn new(base_dir: &str) -> Self {
        let mut foundry_toml_parser = Self {
            path: Path::new(base_dir).join("foundry.toml"),
            remappings: HashMap::<String, String>::new(),
            evm_version: None,
        };

        // Parse the toml file and return it
        foundry_toml_parser.parse_toml();
        foundry_toml_parser
    }

    /// Helper to break apart a remapping gracefully
    fn split_remappings(remapping: &str) -> Option<(String, String)> {
        let mut split = remapping.splitn(2, '=');
        match split.next() {
            Some(from) => split.next().map(|to| (from.to_string(), to.to_string())),
            None => None,
        }
    }

    fn parse_toml(&mut self) {
        match File::open(&self.path) {
            Ok(f) => {
                // Open the buffered reader and read foundry.toml
                let mut data = String::new();
                let mut br = BufReader::new(f);

                // Gracefully read foundry.toml
                if let Err(e) = br.read_to_string(&mut data) {
                    tracing::warn!(target: "utils", "Failed to read \"foundry.toml\" file contents!\nError: {:?}", e);
                    return
                }

                // Parse toml to get the remappings
                let toml = if let Ok(t) = data.parse::<toml::Value>() {
                    t
                } else {
                    tracing::warn!(target: "utils", "\"foundry.toml\" incorrectly formatted!");
                    return
                };

                // Parse the toml as a map
                let toml_map = toml.as_table().cloned().unwrap_or_else(toml::value::Map::new);

                // Transform the mappings into profiles
                let profiles = toml_map
                    .iter()
                    .filter_map(|p| p.1.as_table())
                    .collect::<Vec<&toml::value::Map<String, toml::Value>>>();
                let unwrapped_profiles = profiles
                    .iter()
                    .flat_map(|t| t.values().collect_vec())
                    .collect::<Vec<&toml::Value>>();

                // Extract the inner tables from each profile
                let inner_tables = unwrapped_profiles
                    .iter()
                    .filter_map(|t| t.as_table())
                    .collect::<Vec<&toml::value::Map<String, toml::Value>>>();

                let unwrapped_inner_remappings = inner_tables
                    .iter()
                    .flat_map(|t| {
                        t.into_iter()
                            .filter(|m| m.0.eq("remappings") || m.0.eq("evm_version"))
                            .map(|m| m.1)
                            .collect_vec()
                    })
                    .collect::<Vec<&toml::Value>>();

                // Strings will be the evm version
                let evm_versions = unwrapped_inner_remappings
                    .iter()
                    .filter_map(|t| t.as_str())
                    .collect::<Vec<&str>>();
                self.evm_version = evm_versions.first().map(|s| s.to_string());

                // Extract mappings that are arrays
                let arr_mappings = unwrapped_inner_remappings
                    .iter()
                    .filter_map(|t| t.as_array())
                    .collect::<Vec<&Vec<toml::Value>>>();
                let unwrapped_mappings =
                    arr_mappings.iter().cloned().flatten().collect::<Vec<&toml::Value>>();

                // Filter the remappings as strings
                let remapping_strings =
                    unwrapped_mappings.iter().filter_map(|t| t.as_str()).collect::<Vec<&str>>();

                // For each remapping string, try to split it and insert it into the remappings
                remapping_strings.iter().for_each(|remapping| {
                    match FoundryConfig::split_remappings(remapping) {
                        Some((from, to)) => {
                            self.remappings.insert(from, to);
                        }
                        None => tracing::warn!(target: "utils", "Failed to split remapping using \"=\" at \"{}\" in \"{}\"!", remapping, "fixme"),
                    }
                });
            }

            Err(e) => {
                tracing::warn!(target: "utils", "Foundry.toml not found");
                // tracing::warn!(target: "utils", "Foundry.toml not found in specified \"{}\"",
                // self.path.to_string());
                tracing::warn!(target: "utils", "{:?}", e);
            }
        }
    }

    pub fn parse_remappings() {}
}
