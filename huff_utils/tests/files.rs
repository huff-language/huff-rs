use huff_utils::files::FileSource;

#[test]
fn test_derive_dir() {
    let localized = FileSource::derive_dir("./examples/ERC20.huff").unwrap();
    assert_eq!(localized, "./examples");
    let localized = FileSource::derive_dir("./ERC20.huff").unwrap();
    assert_eq!(localized, ".");
    let localized = FileSource::derive_dir("ERC20.huff").unwrap();
    assert_eq!(localized, "");
}

#[test]
fn test_localize_file() {
    let localized =
        FileSource::localize_file("./examples/ERC20.huff", "./utilities/Address.huff").unwrap();
    assert_eq!(localized, "./examples/utilities/Address.huff");
    let localized = FileSource::localize_file("./ERC20.huff", "./utilities/Address.huff").unwrap();
    assert_eq!(localized, "./utilities/Address.huff");
    let localized = FileSource::localize_file("ERC20.huff", "./utilities/Address.huff").unwrap();
    assert_eq!(localized, "./utilities/Address.huff");
    let localized = FileSource::localize_file("ERC20.huff", "./Address.huff").unwrap();
    assert_eq!(localized, "./Address.huff");
    let localized = FileSource::localize_file("ERC20.huff", "Address.huff").unwrap();
    assert_eq!(localized, "./Address.huff");
    let localized = FileSource::localize_file("./ERC20.huff", "Address.huff").unwrap();
    assert_eq!(localized, "./Address.huff");
    let localized = FileSource::localize_file("./examples/ERC20.huff", "Address.huff").unwrap();
    assert_eq!(localized, "./examples/Address.huff");
    let localized = FileSource::localize_file("./examples/ERC20.huff", "../Address.huff").unwrap();
    assert_eq!(localized, "./Address.huff");
    let localized =
        FileSource::localize_file("./examples/ERC20.huff", "../../Address.huff").unwrap();
    assert_eq!(localized, "../Address.huff");
    let localized =
        FileSource::localize_file("./examples/ERC20.huff", "../../../Address.huff").unwrap();
    assert_eq!(localized, "../../Address.huff");
    let localized =
        FileSource::localize_file("../examples/ERC20.huff", "../../../Address.huff").unwrap();
    assert_eq!(localized, "../../../Address.huff");
    let localized = FileSource::localize_file("../examples/ERC20.huff", "./Address.huff").unwrap();
    assert_eq!(localized, "../examples/Address.huff");
    let localized = FileSource::localize_file("../examples/ERC20.huff", "Address.huff").unwrap();
    assert_eq!(localized, "../examples/Address.huff");
    let localized = FileSource::localize_file("../../examples/ERC20.huff", "Address.huff").unwrap();
    assert_eq!(localized, "../../examples/Address.huff");
    let localized =
        FileSource::localize_file("../../examples/ERC20.huff", "../Address.huff").unwrap();
    assert_eq!(localized, "../../Address.huff");
    let localized =
        FileSource::localize_file("../../examples/ERC20.huff", "../../../Address.huff").unwrap();
    assert_eq!(localized, "../../../../Address.huff");
}
