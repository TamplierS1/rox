use super::err;
pub use std::fmt::{self, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub enum Value
{
    Double(f64),
}

impl fmt::Display for Value
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        let value = match self
        {
            Value::Double(x) => x,
        };

        write!(f, "{}", value)?;

        Ok(())
    }
}

impl Add for Value
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        if let Value::Double(rhs) = rhs
        {
            return match self
            {
                Value::Double(value) => Value::Double(rhs + value),
            };
        }

        err::error(err::Error::RuntimeError(
            "only numbers can be added together.".to_string(),
        ));

        unreachable!()
    }
}

impl Sub for Value
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output
    {
        if let Value::Double(rhs) = rhs
        {
            return match self
            {
                Value::Double(value) => Value::Double(rhs - value),
            };
        }

        err::error(err::Error::RuntimeError(
            "only numbers can be subtracted.".to_string(),
        ));

        unreachable!()
    }
}

impl Mul for Value
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output
    {
        if let Value::Double(rhs) = rhs
        {
            return match self
            {
                Value::Double(value) => Value::Double(rhs * value),
            };
        }

        err::error(err::Error::RuntimeError(
            "only numbers can be multiplied together.".to_string(),
        ));

        unreachable!()
    }
}

impl Div for Value
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output
    {
        if let Value::Double(rhs) = rhs
        {
            return match self
            {
                Value::Double(value) => Value::Double(rhs / value),
            };
        }

        err::error(err::Error::RuntimeError(
            "only numbers can be divided.".to_string(),
        ));

        unreachable!()
    }
}

impl Neg for Value
{
    type Output = Self;

    fn neg(self) -> Self::Output
    {
        match self
        {
            Value::Double(value) => Value::Double(-value),
            _ =>
            {
                err::error(err::Error::RuntimeError(
                    "only numbers can be negated.".to_string(),
                ));

                unreachable!()
            }
        }
    }
}
