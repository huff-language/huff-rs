use ethers_core::types::{Address, U256};
use huff_tests::prelude::{TestRunner, TestStatus};

#[test]
fn test_runner_return() {
    let mut runner = TestRunner::default();
    let code = "602060005260206000F3";
    let deployed_addr = runner.deploy_code(code.to_string()).unwrap();
    let result = runner
        .call(
            String::from("RETURN"),
            Address::zero(),
            deployed_addr,
            U256::zero(),
            String::default(),
        )
        .unwrap();

    assert_eq!(result.name, "RETURN");
    assert_eq!(
        std::mem::discriminant(&result.status),
        std::mem::discriminant(&TestStatus::Success)
    );
    assert_eq!(result.gas, 18);
    assert_eq!(
        result.return_data,
        Some("0000000000000000000000000000000000000000000000000000000000000020".to_string())
    );
}

#[test]
fn test_runner_stop() {
    let mut runner = TestRunner::default();
    let code = "00";
    let deployed_addr = runner.deploy_code(code.to_string()).unwrap();
    let result = runner
        .call(String::from("STOP"), Address::zero(), deployed_addr, U256::zero(), String::default())
        .unwrap();

    assert_eq!(result.name, "STOP");
    assert_eq!(
        std::mem::discriminant(&result.status),
        std::mem::discriminant(&TestStatus::Success)
    );
    assert_eq!(result.gas, 0);
    assert_eq!(result.return_data, None);
}

#[test]
fn test_runner_revert() {
    let mut runner = TestRunner::default();
    let code = "60006000FD";
    let deployed_addr = runner.deploy_code(code.to_string()).unwrap();
    let result = runner
        .call(
            String::from("REVERT"),
            Address::zero(),
            deployed_addr,
            U256::zero(),
            String::default(),
        )
        .unwrap();

    assert_eq!(result.name, "REVERT");
    assert_eq!(std::mem::discriminant(&result.status), std::mem::discriminant(&TestStatus::Revert));
    assert_eq!(result.gas, 6);
    assert_eq!(result.return_data, None);
}
