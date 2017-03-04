use std::collections::HashMap;
use std::io::{self, Write};

use runtime::instruction::*;

pub enum RuntimeError {
    CannotLoad,
}

pub struct RuntimeValues {
    last_modified: u32,
    current_index: u32,
}

// MinumRuntime
pub struct Runtime {
    values: Vec<i64>,
    program: Vec<Instruction>,
    labels: HashMap<Label, u32>,
    instruction_pointer: usize,
    runtime_values: RuntimeValues,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            values: vec![],
            program: vec![],
            labels: HashMap::new(),
            instruction_pointer: 0,
            runtime_values: RuntimeValues {
                last_modified: 0,
                current_index: 0,
            },
        }
    }

    pub fn load(&mut self, code: &str) -> Result<(), RuntimeError> {
        // create array of bytes from code &str
        let bytes = code.as_bytes();
        let start_bytes: [u8; 6] = ['m' as u8, 'i' as u8, 'n' as u8, 'i' as u8, 'm' as u8, 0];
        if bytes[0..6].eq(&start_bytes) {
            // code is bytecode

        } else {
            // code is ascii code

        }

        Result::Ok(())
    }

    pub fn load_instructions(&mut self, code: Vec<Instruction>) {
        self.program = code.to_vec();
    }

    pub fn execute(&mut self) {
        // loop through program and execute each instruction
        loop {
            match self.next() {
                Some(x) => self.execute_instruction(x),
                None => break,
            }
        }
    }

    pub fn next(&mut self) -> Option<Instruction> {
        if self.instruction_pointer >= self.program.len() {
            // reached end of program
            Option::None
        } else {
            let inst = self.program[self.instruction_pointer].clone();
            self.instruction_pointer += 1;
            Option::Some(inst)
        }
    }

    pub fn execute_instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::AssignCell(a, b) => self.assign_cell(a, b),
            Instruction::Output(a, b) => self.output(a, b),
            Instruction::Input(a, b) => self.input(a, b),
            Instruction::CreateLabel(a) => self.create_label(a),
            Instruction::GotoLabel(a) => self.goto_label(a),
            Instruction::TernaryAssign(a, b, c, d, e, f) => self.ternary_assign(a, b, c, d, e, f),
            Instruction::TernaryGoto(a, b, c, d, e, f) => self.ternary_goto(a, b, c, d, e, f),
        }
    }

    pub fn assign_cell(&mut self, cell: MemAccess, value: Value) {
        let cell_index = self.get_mem_value(Box::new(cell)) as usize;
        self.runtime_values.current_index = cell_index as u32;
        let v = self.get_value(Box::new(value));
        if cell_index >= self.values.len() {
            self.values.reserve(cell_index*2);
        }
        let ret = self.values.insert(cell_index, v);
        self.runtime_values.last_modified = cell_index as u32;
        ret
    }

    pub fn output(&mut self, value_type: ValueType, value: Value) {
        match value_type {
            ValueType::Unsigned => {
                let v = self.get_value(Box::new(value)) as u32;
                print!("{}", v);
            },
            ValueType::Signed => {
                let v = self.get_value(Box::new(value)) as i32;
                print!("{}", v);
            },
            ValueType::Ascii => {
                // hacky
                let v = self.get_value(Box::new(value)) as u8 as char;
                print!("{}", v);
            },
        };

    }

    pub fn input(&mut self, value_type: ValueType, value: Value) {

    }

    pub fn create_label(&mut self, label: Label) {

    }

    pub fn goto_label(&mut self, label: Label) {

    }

    pub fn ternary_assign(&mut self, cell: MemAccess, chk0: Value, eq: Equality, chk1: Value, v0: Value, v1: Value) {

    }

    pub fn ternary_goto(&mut self, label: Label, chk0: Value, eq: Equality, chk1: Value, v0: Value, v1: Value) {

    }

    // Helper methods
    fn get_value(&mut self, value: Box<Value>) -> i64 {
        let v = *value;
        match v {
            Value::Mem(a) => self.get_mem_value(Box::new(a)),
            Value::Constant(i) => i,
            Value::Literal(l) => self.get_literal_value(l),
            Value::Add(a, b) => self.get_value(a) + self.get_value(b),
            Value::Sub(a, b) => self.get_value(a) - self.get_value(b),
            Value::Multiply(a, b) => self.get_value(a) * self.get_value(b),
            Value::Divide(a, b) => self.get_value(a) / self.get_value(b),
        }

    }

    fn get_mem_value(&self, mem: Box<MemAccess>) -> i64 {
        let m = *mem;
        match m {
            MemAccess::Index(i) => self.values[i as usize],
            MemAccess::CellNum(i) => i as i64,
            MemAccess::Cell(a) => self.get_mem_value(a),
        }
    }

    fn get_literal_value(&self, l: Literal) -> i64 {
        match l {
            Literal::True => 1,
            Literal::False => 0,
            Literal::Index => self.runtime_values.current_index as i64,
            Literal::LastCell => self.runtime_values.last_modified as i64,
        }
    }
}
