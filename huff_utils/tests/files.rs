use std::sync::Arc;

use huff_utils::{files, prelude::Span};
use tracing_subscriber::EnvFilter;

#[test]
fn test_generate_remappings() {
    let subscriber_builder = tracing_subscriber::fmt();
    let env_filter = EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into());
    if let Err(e) = subscriber_builder.with_env_filter(env_filter).try_init() {
        eprintln!("Failed to initialize tracing!\nError: {e:?}")
    }

    let remapper = files::Remapper::new("../");
    assert_eq!(remapper.remappings.len(), 1);
    assert_eq!(remapper.remappings.get("examples/").unwrap(), "huff-examples/");
}

#[test]
fn test_remappings_from_file() {
    let remapper = files::Remapper::new("./tests");
    assert_eq!(remapper.remappings.len(), 2);
    assert_eq!(remapper.remappings.get("@huffmate/").unwrap(), "lib/huffmate/src/");
    assert_eq!(
        remapper.remappings.get("@openzeppelin/").unwrap(),
        "lib/openzeppelin-contracts/contracts/"
    );
}

#[test]
fn test_source_seg() {
    let span = Span {
        start: 59,
        end: 67,
        file: Some(Arc::new(
            files::FileSource {
                id: uuid::Uuid::nil(),
                path: "./huff-examples/errors/error.huff".to_string(),
                source: Some("#include \"./import.huff\"\n\n#define function addressGetter() internal returns (address)".to_string()),
                access: None,
                dependencies: Some(vec![
                    Arc::new(files::FileSource {
                        id: uuid::Uuid::nil(),
                        path: "./huff-examples/errors/import.huff".to_string(),
                        source: Some("#define macro SOME_RANDOM_MACRO() = takes(2) returns (1) {\n    // Store the keys in memory\n    dup1 0x00 mstore\n    swap1 dup1 0x00 mstore\n\n    // Hash the data, generating a key.\n    0x40 sha3\n}\n".to_string()),
                        access: None,
                        dependencies: Some(vec![])
                    })
                ])
            }
        ))
    };

    let source_seg = span.source_seg();
    assert_eq!(
        source_seg,
        format!(
            "\n     {}|\n  > {} | {}\n     {}|",
            " ", 3, "#define function addressGetter() internal returns (address)", " ",
        )
    );
}

#[test]
fn test_derive_dir() {
    let localized = files::FileSource::derive_dir("./examples/ERC20.huff").unwrap();
    assert_eq!(localized, "./examples");
    let localized = files::FileSource::derive_dir("./ERC20.huff").unwrap();
    assert_eq!(localized, ".");
    let localized = files::FileSource::derive_dir("ERC20.huff").unwrap();
    assert_eq!(localized, "");
}

#[test]
fn test_localize_file() {
    let localized =
        files::FileSource::localize_file("./examples/ERC20.huff", "./utilities/Address.huff")
            .unwrap();
    assert_eq!(localized, "./examples/utilities/Address.huff");
    let localized =
        files::FileSource::localize_file("./ERC20.huff", "./utilities/Address.huff").unwrap();
    assert_eq!(localized, "./utilities/Address.huff");
    let localized =
        files::FileSource::localize_file("ERC20.huff", "./utilities/Address.huff").unwrap();
    assert_eq!(localized, "./utilities/Address.huff");
    let localized = files::FileSource::localize_file("ERC20.huff", "./Address.huff").unwrap();
    assert_eq!(localized, "./Address.huff");
    let localized = files::FileSource::localize_file("ERC20.huff", "Address.huff").unwrap();
    assert_eq!(localized, "./Address.huff");
    let localized = files::FileSource::localize_file("./ERC20.huff", "Address.huff").unwrap();
    assert_eq!(localized, "./Address.huff");
    let localized =
        files::FileSource::localize_file("./examples/ERC20.huff", "Address.huff").unwrap();
    assert_eq!(localized, "./examples/Address.huff");
    let localized =
        files::FileSource::localize_file("./examples/ERC20.huff", "../Address.huff").unwrap();
    assert_eq!(localized, "./Address.huff");
    let localized =
        files::FileSource::localize_file("./examples/ERC20.huff", "../../Address.huff").unwrap();
    assert_eq!(localized, "../Address.huff");
    let localized =
        files::FileSource::localize_file("./examples/ERC20.huff", "../../../Address.huff").unwrap();
    assert_eq!(localized, "../../Address.huff");
    let localized =
        files::FileSource::localize_file("../examples/ERC20.huff", "../../../Address.huff")
            .unwrap();
    assert_eq!(localized, "../../../Address.huff");
    let localized =
        files::FileSource::localize_file("../examples/ERC20.huff", "./Address.huff").unwrap();
    assert_eq!(localized, "../examples/Address.huff");
    let localized =
        files::FileSource::localize_file("../examples/ERC20.huff", "Address.huff").unwrap();
    assert_eq!(localized, "../examples/Address.huff");
    let localized =
        files::FileSource::localize_file("../../examples/ERC20.huff", "Address.huff").unwrap();
    assert_eq!(localized, "../../examples/Address.huff");
    let localized =
        files::FileSource::localize_file("../../examples/ERC20.huff", "../Address.huff").unwrap();
    assert_eq!(localized, "../../Address.huff");
    let localized =
        files::FileSource::localize_file("../../examples/ERC20.huff", "../../../Address.huff")
            .unwrap();
    assert_eq!(localized, "../../../../Address.huff");
    let localized =
        files::FileSource::localize_file("examples/ERC20.huff", "../random_dir/Address.huff")
            .unwrap();
    assert_eq!(localized, "./random_dir/Address.huff");
}
