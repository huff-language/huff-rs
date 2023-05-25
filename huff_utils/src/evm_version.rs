use std::cmp::PartialOrd;

/// Evm Version
///
/// Determines which features will be available when compiling.

#[derive(Debug, PartialEq, PartialOrd)]
enum SupportedEVMVersions {
    paris,
    shanghai,
}

struct EVMVersion {
    version: SupportedEVMVersions,
}

impl EVMVersion {
    fn new(version: SupportedEVMVersions) -> Self {
        Self { version }
    }
}

impl Default for EVMVersion {
    fn default() -> Self {
        Self::new(SupportedEVMVersions::shanghai)
    }
}
