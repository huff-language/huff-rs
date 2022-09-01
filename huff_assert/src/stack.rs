use revm::{Database, EVMData, Inspector, Interpreter, Return, Stack};

#[derive(Debug, Default)]
pub struct StackInspector {
    // stack: Vec<Stack>
}

impl<DB: Database> Inspector<DB> for StackInspector {
    fn initialize_interp(
        &mut self,
        _interp: &mut Interpreter,
        _data: &mut EVMData<'_, DB>,
        _is_static: bool,
    ) -> Return {
        todo!()
    }

    // TODO: Make verification if has stack assertion right after
    fn step_end(
        &mut self,
        _interp: &mut Interpreter,
        _data: &mut EVMData<'_, DB>,
        _is_static: bool,
        _eval: Return,
    ) -> Return {
        todo!()
    }
}
