extern crate minim;

use minim::*;

fn main() {
    let mut program = Vec::new();
    program.push(Instruction::AssignCell(MemAccess::CellNum(0), Value::Constant(42)));
    program.push(Instruction::Output(ValueType::Unsigned, Value::Mem(MemAccess::Index(0))));

    let mut runtime = Runtime::new();
    runtime.load_instructions(program);
    runtime.execute();
}
