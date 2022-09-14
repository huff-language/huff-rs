use huff_tests::types::TestStatus;

/// A stack assertion result
#[derive(Debug, Clone)]
pub struct AssertResult {
    pub name: String,
    pub status: TestStatus,
    pub errors: Vec<String>,
}
