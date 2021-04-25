use super::chunk::{Chunk, Instruction};
use super::error::*;
use super::value::Value;
use std::env;
use std::io::{self, Write};

#[derive(Default)]
pub struct Vm
{
    chunk: Chunk,
}

impl Vm
{
    pub fn init() -> Self
    {
        Default::default()
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), err::Error>
    {
        self.chunk = chunk;
        self.run()
    }

    fn run(&mut self) -> Result<(), err::Error>
    {
        let is_backtrace_on = Vm::is_backtrace_enabled();

        let code = self.chunk.code.clone();
        for instr in code.iter()
        {
            if is_backtrace_on
            {
                self.chunk.print_stack()?;
            }

            match instr
            {
                Instruction::Constant {
                    op: _,
                    line: _,
                    offset,
                } =>
                {
                    let constant = self.read_constant(*offset as usize);
                    self.chunk.stack.push(constant);

                    write!(io::stdout(), "{}\n", constant)?;
                }
                Instruction::Add { .. } =>
                {
                    let b = self.pop_from_stack()?;
                    let a = self.pop_from_stack()?;
                    self.chunk.stack.push(a + b);
                }
                Instruction::Subtract { .. } =>
                {
                    let b = self.pop_from_stack()?;
                    let a = self.pop_from_stack()?;
                    self.chunk.stack.push(a - b);
                }
                Instruction::Multiply { .. } =>
                {
                    let b = self.pop_from_stack()?;
                    let a = self.pop_from_stack()?;
                    self.chunk.stack.push(a * b);
                }
                Instruction::Divide { .. } =>
                {
                    let b = self.pop_from_stack()?;
                    let a = self.pop_from_stack()?;
                    self.chunk.stack.push(a / b);
                }
                Instruction::Negate { .. } =>
                {
                    let value = self.pop_from_stack()?;
                    self.chunk.stack.push(-value);
                }
                Instruction::Return { .. } => match self.chunk.stack.pop()
                {
                    Some(value) => write!(io::stdout(), "{}\n", value)?,
                    None => write!(io::stdout(), "<empty>\n")?,
                },
            };
        }

        if is_backtrace_on
        {
            self.chunk.disassemble("backtrace_chunk");
        }

        Ok(())
    }

    fn read_constant(&self, index: usize) -> Value
    {
        self.chunk.constants[index]
    }

    fn pop_from_stack(&mut self) -> Result<Value, err::Error>
    {
        let value = match self.chunk.stack.pop()
        {
            Some(value) => value,
            None =>
            {
                return Err(err::Error::RuntimeError(String::from(
                    "failed to get a value from the stack. The stack is empty.",
                )));
            }
        };

        Ok(value)
    }

    fn is_backtrace_enabled() -> bool
    {
        if let Ok(_) = env::var("ROX_TRACE_EXECUTION")
        {
            return true;
        }

        false
    }
}
