use bytes::Bytes;
use ethers::prelude::{Address, H256};
use revm::{CallInputs, CreateInputs, Database, EVMData, Gas, Inspector, Return};

#[derive(Debug, Default)]
pub struct SimpleInspector();

impl<DB> Inspector<DB> for SimpleInspector
where
    DB: Database,
{
    fn log(&mut self, _: &mut EVMData<'_, DB>, _: &Address, _: &[H256], _: &Bytes) {
        unimplemented!()
    }

    fn call(
        &mut self,
        _: &mut EVMData<'_, DB>,
        call: &mut CallInputs,
        _: bool,
    ) -> (Return, Gas, Bytes) {
        (Return::Continue, Gas::new(call.gas_limit), Bytes::new())
    }

    fn call_end(
        &mut self,
        _: &mut EVMData<'_, DB>,
        _: &CallInputs,
        gas: Gas,
        status: Return,
        retdata: Bytes,
        _: bool,
    ) -> (Return, Gas, Bytes) {
        (status, gas, retdata)
    }

    fn create(
        &mut self,
        _: &mut EVMData<'_, DB>,
        call: &mut CreateInputs,
    ) -> (Return, Option<Address>, Gas, Bytes) {
        (Return::Continue, None, Gas::new(call.gas_limit), Bytes::new())
    }

    fn create_end(
        &mut self,
        _: &mut EVMData<'_, DB>,
        _: &CreateInputs,
        status: Return,
        address: Option<Address>,
        gas: Gas,
        retdata: Bytes,
    ) -> (Return, Option<Address>, Gas, Bytes) {
        (status, address, gas, retdata)
    }
}
