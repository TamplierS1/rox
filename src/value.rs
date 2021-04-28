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

macro_rules! arithmetic_op_impl {
    ($op:ident, $err_msg:literal) => {
        type Output = Self;

        fn $op(self, rhs: Self) -> Self::Output
        {
            if let Value::Double(rhs) = rhs
            {
                return match self
                {
                    Value::Double(value) => Value::Double(rhs.$op(value)),
                };
            }

            err::error(err::Error::RuntimeError($err_msg.to_string()));

            Value::Double(0.0)
        }
    };
}

impl Add for Value
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        // TODO: Add strings concatenation here later
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

        Value::Double(0.0)
    }
}

impl Sub for Value
{
    arithmetic_op_impl!(sub, "only numbers can be subtracted.");
}

impl Mul for Value
{
    arithmetic_op_impl!(mul, "only numbers can be multiplied together.");
}

impl Div for Value
{
    arithmetic_op_impl!(div, "only numbers can be divided together.");
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

                Value::Double(0.0)
            }
        }
    }
}
