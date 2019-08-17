use crate::value::{ConstantPool, Value};

#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    Constant(usize),
    Return,
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub op_code: OpCode,
    pub line: usize,
}

impl Instruction {
    pub fn new(op_code: OpCode, line: usize) -> Self {
        Self { op_code, line }
    }
}

pub struct Chunk {
    code: Vec<Instruction>,
    constants: ConstantPool,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: ConstantPool::new(),
        }
    }

    pub fn add_instruction(&mut self, inst: Instruction) {
        self.code.push(inst);
    }

    pub fn add_constant_instuction(&mut self, value: Value, line: usize) {
        let idx = self.add_constant(value);
        self.add_instruction(Instruction::new(OpCode::Constant(idx), line));
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, idx: usize) -> Option<Value> {
        if idx < self.constants.len() {
            Some(self.constants[idx])
        } else {
            None
        }
    }

    pub fn get_code(&self) -> &Vec<Instruction> {
        &self.code
    }
}
