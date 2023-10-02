use std::convert::TryFrom;

// use crate::configuration::collection::{Map, Property, Routine};

#[derive(Debug)]
pub enum ValueError {
    ConversionError,
}

#[derive(Debug)]
pub enum Value {
    Int(i128),
    Uint(u128),
    Float(f64),
    String(String),
    Boolean(bool),
    // Array(Vec<Value>),
    // Dictionary(Box<dyn Map<String, Value, Property>>),
    // Tuple(Vec<Value>),
    // Routine(Routine),
}

// TODO: TryFrom to just From (I think)
// TODO: Implement Dictionary, Tuple, and Routine

impl TryFrom<i128> for Value {
    type Error = ValueError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        Ok(Value::Int(value))
    }
}

impl TryFrom<&Value> for i128 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => Ok(*i),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<Value> for i128 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => Ok(i),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<i64> for Value {
    type Error = ValueError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Value::Int(value as i128))
    }
}

impl TryFrom<Value> for i64 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => i.try_into().map_err(|_| ValueError::ConversionError),
            _ => Err(ValueError::ConversionError),
        }
    }
}
impl TryFrom<&Value> for i64 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => Ok(*i as i64),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<i32> for Value {
    type Error = ValueError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Value::Int(value as i128))
    }
}

impl TryFrom<Value> for i32 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => i.try_into().map_err(|_| ValueError::ConversionError),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<&Value> for i32 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => Ok(*i as i32),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<i16> for Value {
    type Error = ValueError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(Value::Int(value as i128))
    }
}

impl TryFrom<Value> for i16 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => i.try_into().map_err(|_| ValueError::ConversionError),
            _ => Err(ValueError::ConversionError),
        }
    }
}
impl TryFrom<&Value> for i16 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => Ok(*i as i16),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<i8> for Value {
    type Error = ValueError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(Value::Int(value as i128))
    }
}

impl TryFrom<Value> for i8 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => i.try_into().map_err(|_| ValueError::ConversionError),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<&Value> for i8 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(i) => Ok(*i as i8),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<u128> for Value {
    type Error = ValueError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value))
    }
}

impl TryFrom<Value> for u128 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => Ok(u),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<&Value> for u128 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => Ok(*u),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<u64> for Value {
    type Error = ValueError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value as u128))
    }
}

impl TryFrom<Value> for u64 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => u.try_into().map_err(|_| ValueError::ConversionError),
            _ => Err(ValueError::ConversionError),
        }
    }
}
impl TryFrom<&Value> for u64 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => Ok(*u as u64),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<u32> for Value {
    type Error = ValueError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value as u128))
    }
}

impl TryFrom<Value> for u32 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => u.try_into().map_err(|_| ValueError::ConversionError),
            _ => Err(ValueError::ConversionError),
        }
    }
}
impl TryFrom<&Value> for u32 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => Ok(*u as u32),
            _ => Err(ValueError::ConversionError),
        }
    }
}
impl TryFrom<u16> for Value {
    type Error = ValueError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value as u128))
    }
}

impl TryFrom<Value> for u16 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => u.try_into().map_err(|_| ValueError::ConversionError),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<&Value> for u16 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => Ok(*u as u16),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<u8> for Value {
    type Error = ValueError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value as u128))
    }
}

impl TryFrom<Value> for u8 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => u.try_into().map_err(|_| ValueError::ConversionError),
            _ => Err(ValueError::ConversionError),
        }
    }
}
impl TryFrom<&Value> for u8 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Uint(u) => Ok(*u as u8),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<f64> for Value {
    type Error = ValueError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Value::Float(value as f64))
    }
}

impl TryFrom<Value> for f64 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(f) => Ok(f),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<&Value> for f64 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(f) => Ok(*f),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<f32> for Value {
    type Error = ValueError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Value::Float(value as f64))
    }
}

impl TryFrom<Value> for f32 {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(f) => Ok(f as f32),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<&Value> for f32 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(f) => Ok(*f as f32),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<String> for Value {
    type Error = ValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Value::String(value))
    }
}

impl TryFrom<Value> for String {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<&Value> for String {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s.clone()),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<&str> for Value {
    type Error = ValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Value::String(value.to_string()))
    }
}

// Implement for &String instead of &str because of lifetimes
impl<'a> TryFrom<&'a Value> for &'a String {
    type Error = ValueError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(ValueError::ConversionError),
        }
    }
}

// This is impossible due to lifetime issues. See the above block for the equivilent
// impl TryFrom<Value> for &str {
//     type Error = ValueError;
//
//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         match &value {
//             Value::String(s) => Ok(s.as_str()),
//             _ => Err(ValueError::ConversionError),
//         }
//     }
// }

impl<'a> TryFrom<&'a Value> for &'a str {
    type Error = ValueError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<bool> for Value {
    type Error = ValueError;

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        Ok(Value::Boolean(value))
    }
}

impl TryFrom<Value> for bool {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(b) => Ok(b),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl TryFrom<&Value> for bool {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(b) => Ok(*b),
            _ => Err(ValueError::ConversionError),
        }
    }
}
// impl<T: Into<Value>> TryFrom<Vec<T>> for Value {
//     type Error = ValueError;
//
//     fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
//         Ok(Value::Array(value.into_iter().map(T::into).collect()))
//     }
// }
//
// impl<T: Into<Value>, const N: usize> TryFrom<[T; N]> for Value {
//     type Error = ValueError;
//
//     fn try_from(value: [T; N]) -> Result<Self, Self::Error> {
//         Ok(Value::Array(
//             value.into_iter().map(|v| v.clone().into()).collect(),
//         ))
//     }
// }
//
// impl<T: Into<Value>> TryFrom<&[T]> for Value {
//     type Error = ValueError;
//
//     fn try_from(value: &[T]) -> Result<Self, Self::Error> {
//         Ok(Value::Array(
//             value.into_iter().map(|v| v.clone().into()).collect(),
//         ))
//     }
// }
