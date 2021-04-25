use super::error::*;
use super::value::*;
use std::io::{self, Write};

#[derive(Copy, Clone)]
pub enum OpCode
{
    Constant = 0,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

#[derive(Copy, Clone)]
pub enum Instruction
{
    Constant
    {
        op: OpCode, line: u32, offset: u32
    },
    Add
    {
        op: OpCode, line: u32
    },
    Subtract
    {
        op: OpCode, line: u32
    },
    Multiply
    {
        op: OpCode, line: u32
    },
    Divide
    {
        op: OpCode, line: u32
    },
    Negate
    {
        op: OpCode, line: u32
    },
    Return
    {
        op: OpCode, line: u32
    },
}

impl Instruction
{
    // For simple instructions that don't have
    // anything besides their names displayed
    pub fn display_simple(
        f: &mut Formatter,
        prev_line: u32,
        line: u32,
        name: &str,
    ) -> Result<u32, err::Error>
    {
        Chunk::print_line(f, line, prev_line)?;
        write!(f, "{}\n", name)?;
        Ok(line)
    }

    pub fn display_constant(
        constants: &Vec<Value>,
        f: &mut Formatter,
        prev_line: u32,
        line: u32,
        offset: u32,
    ) -> Result<u32, err::Error>
    {
        Chunk::print_line(f, line, prev_line)?;
        write!(
            f,
            "OP_CONSTANT {} '{}'\n",
            offset, constants[offset as usize],
        )?;
        Ok(line)
    }
}

// Chunk is a series of instructions
#[derive(Default)]
pub struct Chunk
{
    pub code: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub stack: Vec<Value>,
}

impl fmt::Display for Chunk
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        let mut prev_line: u32 = 0;

        for (index, instruction) in self.code.iter().enumerate()
        {
            write!(f, "{:0>4} ", index)?;

            prev_line = match instruction
            {
                Instruction::Constant {
                    op: _,
                    line,
                    offset,
                } => Instruction::display_constant(&self.constants, f, prev_line, *line, *offset)
                    .unwrap(),
                Instruction::Add { op: _, line } =>
                {
                    Instruction::display_simple(f, prev_line, *line, "OP_ADD").unwrap()
                }
                Instruction::Subtract { op: _, line } =>
                {
                    Instruction::display_simple(f, prev_line, *line, "OP_SUBTRACT").unwrap()
                }
                Instruction::Multiply { op: _, line } =>
                {
                    Instruction::display_simple(f, prev_line, *line, "OP_MULTIPLY").unwrap()
                }
                Instruction::Divide { op: _, line } =>
                {
                    Instruction::display_simple(f, prev_line, *line, "OP_DIVIDE").unwrap()
                }
                Instruction::Negate { op: _, line } =>
                {
                    Instruction::display_simple(f, prev_line, *line, "OP_NEGATE").unwrap()
                }
                Instruction::Return { op: _, line } =>
                {
                    Instruction::display_simple(f, prev_line, *line, "OP_RETURN").unwrap()
                }
            };
        }

        Ok(())
    }
}

impl Chunk
{
    pub fn new() -> Self
    {
        Default::default()
    }

    pub fn write(&mut self, instruction: Instruction)
    {
        self.code.push(instruction);
    }

    pub fn add_constant(&mut self, constant: Value) -> u32
    {
        self.constants.push(constant);
        let index = (self.constants.len() - 1) as u32;
        index
    }

    pub fn disassemble(&self, name: &str)
    {
        println!("\n== {} ==\n{}", name, self);
    }

    pub fn print_stack(&self) -> Result<(), err::Error>
    {
        write!(io::stdout(), "\t")?;
        for value in self.stack.iter()
        {
            write!(io::stdout(), "[{}]", value)?;
        }
        write!(io::stdout(), "\n")?;

        Ok(())
    }

    fn print_line(f: &mut Formatter<'_>, line: u32, prev_line: u32) -> Result<(), err::Error>
    {
        if line == prev_line
        {
            // points out that the previous instruction
            // is located on the same line of source code
            write!(f, "{:^4}", "|")?;
        }
        else
        {
            write!(f, "{:^4}", line)?;
        }

        Ok(())
    }
}
