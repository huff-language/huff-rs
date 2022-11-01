use crate::prelude::Artifact;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Generate solidity interfaces from a vector of artifacts.
///
/// @param artifacts The vector of artifacts to generate interfaces from.
/// @return The vector of generated interfaces.
pub fn gen_sol_interfaces(
    artifacts: &Vec<Arc<Artifact>>,
    interface: Option<String>,
) -> Vec<(PathBuf, String, String)> {
    let mut interfaces = Vec::new();

    for artifact in artifacts {
        if let Some(a) = &artifact.abi {
            let mut defs = Vec::new();
            a.events.iter().for_each(|(_, f)| {
                defs.push(format!(
                    "{}event {}({});",
                    "\t",
                    f.name,
                    f.inputs
                        .iter()
                        .map(|i| {
                            format!(
                                "{}{}",
                                i.kind,
                                if i.indexed {
                                    String::from(" indexed")
                                } else {
                                    String::default()
                                }
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(", "),
                ));
            });
            a.errors.iter().for_each(|(_, e)| {
                defs.push(format!(
                    "{}error {}({});",
                    "\t",
                    e.name,
                    e.inputs
                        .iter()
                        .map(|i| format!(
                            "{}{}",
                            i.kind,
                            if i.kind.is_memory_type() { " memory" } else { "" }
                        ))
                        .collect::<Vec<_>>()
                        .join(", "),
                ));
            });
            a.functions.iter().for_each(|(_, f)| {
                defs.push(format!(
                    "{}function {}({}) external{}{};",
                    "\t",
                    f.name,
                    f.inputs
                        .iter()
                        .map(|i| format!(
                            "{}{}",
                            i.kind,
                            if i.kind.is_memory_type() { " memory" } else { "" }
                        ))
                        .collect::<Vec<_>>()
                        .join(", "),
                    f.state_mutability.interface_mutability(),
                    if f.outputs.is_empty() {
                        String::default()
                    } else {
                        format!(
                            " returns ({})",
                            f.outputs
                                .iter()
                                .map(|o| format!(
                                    "{}{}",
                                    o.kind,
                                    if o.kind.is_memory_type() { " memory" } else { "" }
                                ))
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    },
                ));
            });

            let interface_name = interface.clone().unwrap_or_else(|| {
                format!(
                    "I{}",
                    artifact.file.path.split('/').last().unwrap().split('.').next().unwrap()
                )
            });
            let formatted_str = format!("interface {interface_name} {{\n{}\n}}", defs.join("\n"));
            interfaces.push((
                Path::new(&artifact.file.path).parent().unwrap().to_path_buf(),
                interface_name,
                formatted_str,
            ));
        }
    }

    interfaces
}

/// Export generated solidity interfaces to a file.
///
/// @param interfaces The vector of generated interfaces.
/// @return Unit type if success, error if failure.
pub fn export_interfaces(
    interfaces: &Vec<(PathBuf, String, String)>,
) -> Result<(), std::io::Error> {
    for (path, name, interface) in interfaces {
        let path_str = format!("{}/{name}.sol", path.to_str().unwrap_or(""));
        let file_path = Path::new(&path_str);
        fs::write(file_path, interface)?;
    }
    Ok(())
}
