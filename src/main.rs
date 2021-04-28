#![feature(try_trait)]
#![feature(exclusive_range_pattern)]

mod chunk;
mod compiler;
mod error;
mod scanner;
mod value;
mod vm;

use std::{
    env, fs,
    io::{self, Write},
};

use error::err;
use std::io::Read;
use vm::*;

fn main()
{
    if let Err(e) = run(env::args())
    {
        println!("{}", e);
    }
}

fn run(mut args: env::Args) -> Result<(), err::Error>
{
    let mut vm = Vm::init();

    if args.len() > 2
    {
        return Err(err::Error::RuntimeError("Usage: rox [path]\n".to_string()));
    }

    // Skip the first argument
    args.next();

    match args.next()
    {
        Some(path) => run_script(path)?,
        None => repl()?,
    }

    Ok(())
}

fn repl() -> Result<(), err::Error>
{
    let mut line = String::new();
    loop
    {
        write!(io::stdout(), "> ")?;

        io::stdin().read_to_string(&mut line)?;

        interpret(line.clone());
    }
}

fn run_script(path: String) -> Result<(), err::Error>
{
    let source = fs::read_to_string(path)?;
    if let Err(e) = interpret(source)
    {
        return Err(e);
    }

    Ok(())
}

fn interpret(source: String) -> Result<(), err::Error>
{
    compiler::compile(source)?;
    Ok(())
}
