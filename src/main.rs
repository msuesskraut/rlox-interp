mod disasm;
mod lexer;
mod value;
mod vm;

use disasm::disassemble_chunk;
use value::Value;
use vm::{Chunk, Instruction, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_constant_instuction(Value::Number(42f64), 12);
    chunk.add_constant_instuction(Value::Number(23f64), 12);
    chunk.add_instruction(Instruction::new(OpCode::Return, 13));

    disassemble_chunk(&chunk, "test");
}
