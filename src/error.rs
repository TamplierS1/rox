pub mod err
{
    use std::fmt::{self, Formatter};
    use std::io::{self, Write};

    #[derive(Debug)]
    pub enum Error
    {
        CompileError(String),
        RuntimeError(String),
    }

    pub fn error(error: self::Error)
    {
        // This function should not return anything,
        // that's why I'm using unwrap() here
        write!(io::stderr(), "{}\n", error).unwrap();
    }

    impl fmt::Display for Error
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
        {
            match self
            {
                Self::CompileError(msg) => write!(f, "Compile Error: {}", msg)?,
                Self::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg)?,
            }

            Ok(())
        }
    }

    impl From<std::io::Error> for Error
    {
        fn from(error: std::io::Error) -> Self
        {
            Error::RuntimeError(error.to_string())
        }
    }

    impl From<std::fmt::Error> for Error
    {
        fn from(error: std::fmt::Error) -> Self
        {
            Error::RuntimeError(error.to_string())
        }
    }
}
