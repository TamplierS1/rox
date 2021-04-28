use super::error::err;
use super::scanner::{Scanner, TokenKind};
use std::convert::AsRef;
use std::io::{self, Write};

pub fn compile(source: String) -> Result<(), err::Error>
{
    let mut scanner = Scanner::new(&source);

    let mut current_line = 0;
    loop
    {
        let token = scanner.scan_token()?;
        // If tokens are on the same line
        // it displays the pipe character '|' instead
        // of a line number
        if token.line != current_line
        {
            write!(io::stdout(), "{}", token.line)?;
            current_line = token.line;
        }
        else
        {
            write!(io::stdout(), "{}", "|")?;
        }

        write!(
            io::stdout(),
            "\t{:14} '{}'\n",
            token.kind.as_ref(),
            token.as_str(),
        )?;

        if token.kind == TokenKind::Eof
        {
            break;
        }
    }

    Ok(())
}
