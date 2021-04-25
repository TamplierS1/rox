#![feature(try_trait)]

mod chunk;
mod error;
mod value;
mod vm;

use std::env;

use chunk::*;
use error::*;
use value::Value;
use vm::*;

fn main()
{
    if let Err(e) = run(env::args())
    {
        println!("{}", e);
    }
}

fn run(args: env::Args) -> Result<(), err::Error>
{
    let mut vm = Vm::init();

    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Value::Double(10.5));
    chunk.write(Instruction::Constant {
        op: OpCode::Constant,
        line: 1,
        offset: constant,
    });
    chunk.write(Instruction::Constant {
        op: OpCode::Constant,
        line: 1,
        offset: constant,
    });
    chunk.write(Instruction::Subtract {
        op: OpCode::Subtract,
        line: 1,
    });
    chunk.write(Instruction::Return {
        op: OpCode::Return,
        line: 2,
    });

    vm.interpret(chunk)?;

    Ok(())
}
