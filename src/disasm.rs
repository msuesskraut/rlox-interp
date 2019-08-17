use crate::value::Value;
use crate::vm::{Chunk, Instruction, OpCode};

struct Disassambler<'a> {
    chunk: &'a Chunk,
}

impl<'a> Disassambler<'a> {
    fn new(chunk: &'a Chunk) -> Self {
        Self { chunk }
    }

    fn disassemble(&self) {
        let mut last_line = std::usize::MAX;
        for (offset, inst) in self.chunk.get_code().iter().enumerate() {
            let continue_line = last_line == inst.line;
            self.disassemble_instruction(inst, continue_line, offset);
            last_line = inst.line;
        }
    }

    fn disassemble_instruction(&self, inst: &Instruction, continue_line: bool, offset: usize) {
        use OpCode::*;
        print!("{:04} ", offset);
        if continue_line {
            print!("   | ");
        } else {
            print!("{:4} ", inst.line);
        }

        match inst.op_code {
            Return => println!("OP_RETURN"),
            Constant(idx) => self.constant_instruction("OP_CONSTANT", self.chunk.get_constant(idx)),
        }
    }

    fn constant_instruction(&self, name: &str, value: Option<Value>) {
        println!("{:-16} {:?}", name, value.unwrap_or(Value::Undef))
    }
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    Disassambler::new(chunk).disassemble();
}
