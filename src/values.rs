use std::{collections::HashMap, convert::TryFrom};

use crate::configuration::collection::Routine;

#[derive(Debug)]
pub enum ValueError {
    ConversionError,
}

// TODO: Is there a void or () type

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Int,
    Uint,
    Float,
    String,
    Boolean,
    Array,
    Dictionary,
    // TODO: Leave off Routines for now, unless I want lambdas to be first-class citizens eventually
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i128),
    Uint(u128),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Dictionary(HashMap<String, Value>), // Stick with HashMaps for now, maybe later a more generic Map
    Routine(Routine),
}

// I must use TryFrom and not From because all the of number and collection types are fallible
//////////////////////////// Value

impl TryFrom<&Value> for Value {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Value::from(value.clone()))
    }
}

///////////////////// i128
impl TryFrom<i128> for Value {
    type Error = ValueError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        Ok(Value::Int(value))
    }
}

impl TryFrom<&i128> for Value {
    type Error = ValueError;

    fn try_from(value: &i128) -> Result<Self, Self::Error> {
        Ok(Value::Int(value.clone()))
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

//////////////////// i64
impl TryFrom<i64> for Value {
    type Error = ValueError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Value::Int(value as i128))
    }
}

impl TryFrom<&i64> for Value {
    type Error = ValueError;

    fn try_from(value: &i64) -> Result<Self, Self::Error> {
        Ok(Value::Int(value.clone() as i128))
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

/////////////////// i32
impl TryFrom<i32> for Value {
    type Error = ValueError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Value::Int(value as i128))
    }
}

impl TryFrom<&i32> for Value {
    type Error = ValueError;

    fn try_from(value: &i32) -> Result<Self, Self::Error> {
        Ok(Value::Int(value.clone() as i128))
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

///////////////// i16
impl TryFrom<i16> for Value {
    type Error = ValueError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Ok(Value::Int(value as i128))
    }
}

impl TryFrom<&i16> for Value {
    type Error = ValueError;

    fn try_from(value: &i16) -> Result<Self, Self::Error> {
        Ok(Value::Int(value.clone() as i128))
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

/////////////////// i8
impl TryFrom<i8> for Value {
    type Error = ValueError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(Value::Int(value as i128))
    }
}

impl TryFrom<&i8> for Value {
    type Error = ValueError;

    fn try_from(value: &i8) -> Result<Self, Self::Error> {
        Ok(Value::Int(value.clone() as i128))
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

//////////////////// u128
impl TryFrom<u128> for Value {
    type Error = ValueError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value))
    }
}

impl TryFrom<&u128> for Value {
    type Error = ValueError;

    fn try_from(value: &u128) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value.clone() as u128))
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

///////////////////// u64
impl TryFrom<u64> for Value {
    type Error = ValueError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value as u128))
    }
}

impl TryFrom<&u64> for Value {
    type Error = ValueError;

    fn try_from(value: &u64) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value.clone() as u128))
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

//////////////////// u32
impl TryFrom<u32> for Value {
    type Error = ValueError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value as u128))
    }
}

impl TryFrom<&u32> for Value {
    type Error = ValueError;

    fn try_from(value: &u32) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value.clone() as u128))
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

//////////////////////// u16
impl TryFrom<u16> for Value {
    type Error = ValueError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value as u128))
    }
}

impl TryFrom<&u16> for Value {
    type Error = ValueError;

    fn try_from(value: &u16) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value.clone() as u128))
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

//////////////// u8
impl TryFrom<u8> for Value {
    type Error = ValueError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value as u128))
    }
}

impl TryFrom<&u8> for Value {
    type Error = ValueError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        Ok(Value::Uint(value.clone() as u128))
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

/////////////////// f64
impl TryFrom<f64> for Value {
    type Error = ValueError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Value::Float(value as f64))
    }
}

impl TryFrom<&f64> for Value {
    type Error = ValueError;

    fn try_from(value: &f64) -> Result<Self, Self::Error> {
        Ok(Value::Float(value.clone() as f64))
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

//////////////// f32
impl TryFrom<f32> for Value {
    type Error = ValueError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Value::Float(value as f64))
    }
}

impl TryFrom<&f32> for Value {
    type Error = ValueError;

    fn try_from(value: &f32) -> Result<Self, Self::Error> {
        Ok(Value::Float(value.clone() as f64))
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

/////////////////// String
impl TryFrom<String> for Value {
    type Error = ValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Value::String(value))
    }
}

impl TryFrom<&String> for Value {
    type Error = ValueError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Ok(Value::String(value.clone()))
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

////////////////// &str
impl TryFrom<&str> for Value {
    type Error = ValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Value::String(value.to_string()))
    }
}

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

///////////////////////// bool
impl TryFrom<bool> for Value {
    type Error = ValueError;

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        Ok(Value::Boolean(value))
    }
}

impl TryFrom<&bool> for Value {
    type Error = ValueError;

    fn try_from(value: &bool) -> Result<Self, Self::Error> {
        Ok(Value::Boolean(value.clone()))
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

//////////////////////////////////// Vec<T>
impl<T> TryFrom<Vec<T>> for Value
where
    T: TryInto<Value>,
    <T as TryInto<Value>>::Error: std::fmt::Debug,
{
    type Error = ValueError;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        Ok(Value::Array(
            vec.into_iter()
                .map(|item| item.try_into().unwrap())
                .collect(),
        ))
    }
}

impl<T> TryFrom<&Vec<T>> for Value
where
    T: TryInto<Value> + Clone,
    <T as TryInto<Value>>::Error: std::fmt::Debug,
{
    type Error = ValueError;

    fn try_from(vec: &Vec<T>) -> Result<Self, Self::Error> {
        Ok(Value::Array(
            vec.into_iter()
                .map(|item| item.clone().try_into().unwrap())
                .collect(),
        ))
    }
}

impl<'a, T> TryFrom<&'a Value> for Vec<T>
where
    T: Clone + TryFrom<&'a Value>,
    <T as TryFrom<&'a Value>>::Error: std::fmt::Debug,
{
    type Error = ValueError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(vec) => vec
                .iter()
                .map(|v| T::try_from(v).map_err(|_| ValueError::ConversionError))
                .collect(),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl<T> TryFrom<Value> for Vec<T>
where
    T: Clone + TryFrom<Value>,
    <T as TryFrom<Value>>::Error: std::fmt::Debug,
{
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(vec) => vec
                .iter()
                .map(|v| T::try_from(v.clone()).map_err(|_| ValueError::ConversionError))
                .collect(),
            _ => Err(ValueError::ConversionError),
        }
    }
}

////////////////////////////////// HashMap
impl<V> TryFrom<HashMap<String, V>> for Value
where
    V: TryInto<Value>,
    <V as TryInto<Value>>::Error: std::fmt::Debug,
{
    type Error = ValueError;

    fn try_from(map: HashMap<String, V>) -> Result<Self, Self::Error> {
        Ok(Value::Dictionary(
            map.into_iter()
                .map(|(k, v)| (k.into(), v.try_into().unwrap()))
                .collect(),
        ))
    }
}

impl<V> TryFrom<&HashMap<String, V>> for Value
where
    V: TryInto<Value> + Clone,
    <V as TryInto<Value>>::Error: std::fmt::Debug,
{
    type Error = ValueError;

    fn try_from(map: &HashMap<String, V>) -> Result<Self, Self::Error> {
        Ok(Value::Dictionary(
            map.into_iter()
                .map(|(k, v)| (k.into(), v.clone().try_into().unwrap()))
                .collect(),
        ))
    }
}

impl<'a, V> TryFrom<&'a Value> for HashMap<String, V>
where
    V: Clone + TryFrom<&'a Value>,
    <V as TryFrom<&'a Value>>::Error: std::fmt::Debug,
{
    type Error = ValueError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Dictionary(map) => map
                .iter()
                .map(|(k, v)| {
                    Ok((
                        k.clone(),
                        V::try_from(v).map_err(|_| ValueError::ConversionError)?,
                    ))
                })
                .collect(),
            _ => Err(ValueError::ConversionError),
        }
    }
}

impl<V> TryFrom<Value> for HashMap<String, V>
where
    V: Clone + TryFrom<Value>,
    <V as TryFrom<Value>>::Error: std::fmt::Debug,
{
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Dictionary(map) => map
                .iter()
                .map(|(k, v)| {
                    Ok((
                        k.clone(),
                        V::try_from(v.clone()).map_err(|_| ValueError::ConversionError)?,
                    ))
                })
                .collect(),
            _ => Err(ValueError::ConversionError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::values::Value;
    use maplit::hashmap;
    use std::collections::HashMap;

    // TODO: This can be split into many tests
    // TODO: Use random input instead of fixed
    #[test]
    fn it_handles_value_conversions() {
        // i8
        let value = 100 as i8;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        let value = 100 as i8;
        assert_eq!(
            value,
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        // i16
        let value = 10000 as i16;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        // i32
        let value = 1000000 as i32;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        // i64
        let value = 100000000 as i64;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        // i128
        let value = 10000000000 as i128;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        // u8
        let value = 100 as u8;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        let value = 10000 as u16;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        let value = 10000 as u32;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        let value = 10000 as u64;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        let value = 10000 as u128;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        let value = 10000.1 as f32;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        let value = 10000.1 as f64;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        let value = true;
        assert_eq!(value, Value::try_from(value).unwrap().try_into().unwrap());

        assert_eq!(
            value.clone(),
            (&Value::try_from(value).unwrap()).try_into().unwrap()
        );

        let original = "test string".to_string();
        let built = Value::try_from(original.clone()).unwrap();
        let actual: String = built.try_into().unwrap();
        assert_eq!(original, actual);

        let original = "test string2";
        let built = Value::try_from(original).unwrap();
        let actual: String = built.try_into().unwrap();
        assert_eq!(original, actual);

        let original = "test string3";
        let built = Value::try_from(original).unwrap();
        let actual: &String = &built.try_into().unwrap();
        assert_eq!(original, actual);

        let original = "test string".to_string();
        let built = Value::try_from(original.clone()).unwrap();
        let actual: String = built.try_into().unwrap();
        assert_eq!(original, actual);

        let original = vec![true, false, true];
        let built = Value::try_from(original.clone()).unwrap();
        let actual: Vec<bool> = (&built).try_into().unwrap();
        assert_eq!(original, actual);

        let original = vec![true, false, true];
        let built = Value::try_from(original.clone()).unwrap();
        let actual: Vec<bool> = built.try_into().unwrap();
        assert_eq!(original, actual);

        let original = vec![1, 2, 3];
        let built = Value::try_from(original.clone()).unwrap();
        let actual: Vec<i32> = (&built).try_into().unwrap();
        assert_eq!(original, actual);

        let original = vec![4, 5, 6];
        let built = Value::try_from(original.clone()).unwrap();
        let actual: Vec<i32> = built.try_into().unwrap();
        assert_eq!(original, actual);

        let original = vec!["a", "b", "c"];
        let built = Value::try_from(original.clone()).unwrap();
        let actual: Vec<&str> = (&built).try_into().unwrap();
        assert_eq!(original, actual);

        let original = vec!["d".to_string(), "e".to_string()];
        let built = Value::try_from(original.clone()).unwrap();
        let actual: Vec<String> = built.try_into().unwrap();
        assert_eq!(original, actual);

        let original = vec!["d".to_string(), "e".to_string()];
        let built = Value::try_from(original.clone()).unwrap();
        let actual: Vec<String> = (&built).try_into().unwrap();
        assert_eq!(original, actual);

        // Hash Maps
        let original = hashmap! {
            "A".to_string() => true,
            "B".to_string() => false
        };

        let built = Value::try_from(original.clone()).unwrap();
        let actual: HashMap<String, bool> = (&built).try_into().unwrap();
        assert_eq!(original, actual);

        let original = hashmap! {
            "A".to_string() => true,
            "B".to_string() => false
        };

        let built = Value::try_from(original.clone()).unwrap();
        let actual: HashMap<String, bool> = built.try_into().unwrap();
        assert_eq!(original, actual);

        let original = hashmap! {
            "X".to_string() => 3,
            "Y".to_string() => 8
        };

        let built = Value::try_from(original.clone()).unwrap();
        let actual: HashMap<String, i32> = (&built).try_into().unwrap();
        assert_eq!(original, actual);

        let original = hashmap! {
            "X".to_string() => 1,
            "Y".to_string() => 80
        };

        let built = Value::try_from(original.clone()).unwrap();
        let actual: HashMap<String, i32> = built.try_into().unwrap();
        assert_eq!(original, actual);

        let original = hashmap! {
            "X".to_string() => "michael",
            "Y".to_string() => "brian"
        };

        let built = Value::try_from(original.clone()).unwrap();
        let actual: HashMap<String, &str> = (&built).try_into().unwrap();
        assert_eq!(original, actual);

        let original = hashmap! {
            "X".to_string() => "michael".to_string(),
            "Y".to_string() => "brian".to_string(),
        };

        let built = Value::try_from(original.clone()).unwrap();
        let actual: HashMap<String, String> = built.try_into().unwrap();
        assert_eq!(original, actual);
    }
}
