use ethers::prelude::{Address, U256};
use huff_tests::runner::TestRunner;
use revm::Return;

#[test]
fn test_runner_return() {
    let mut runner = TestRunner::default();
    let code = "602060005260206000F3";
    let deployed_addr = runner.deploy_code(code.to_string()).unwrap();
    let result = runner
        .call(String::from("RETURN"), Address::zero(), deployed_addr, U256::zero(), "")
        .unwrap();

    assert_eq!(result.name, "RETURN");
    assert_eq!(result.status, Return::Return);
    assert_eq!(result.gas, 18);
    assert_eq!(
        result.return_data,
        "0000000000000000000000000000000000000000000000000000000000000020"
    );
}

#[test]
fn test_runner_stop() {
    let mut runner = TestRunner::default();
    let code = "00";
    let deployed_addr = runner.deploy_code(code.to_string()).unwrap();
    let result = runner
        .call(String::from("STOP"), Address::zero(), deployed_addr, U256::zero(), "")
        .unwrap();

    assert_eq!(result.name, "STOP");
    assert_eq!(result.status, Return::Stop);
    assert_eq!(result.gas, 0);
    assert_eq!(result.return_data, "");
}

#[test]
fn test_runner_revert() {
    let mut runner = TestRunner::default();
    let code = "60006000FD";
    let deployed_addr = runner.deploy_code(code.to_string()).unwrap();
    let result = runner
        .call(String::from("REVERT"), Address::zero(), deployed_addr, U256::zero(), "")
        .unwrap();

    assert_eq!(result.name, "REVERT");
    assert_eq!(result.status, Return::Revert);
    assert_eq!(result.gas, 6);
    assert_eq!(result.return_data, "");
}
