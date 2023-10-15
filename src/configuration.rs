use crate::values::Value;
use std::{any::Any, collections::HashMap};

pub mod collection;

pub type ConfigurationEntries = HashMap<String, Property>;

// #[derive(Debug, PartialEq, Clone)]
// Cannot derive these here because Box<dyn Configuration + etc> is not valid
pub enum Property {
    Entry {
        default: Value,
        value: Option<Value>,
        // validations: Vec<Validation>
    },
    Nested {
        value: Box<dyn Configuration>,
    },
}

// impl Property {
//     pub fn new(name: &str, path: &str, default: Value) -> Self {
//         Self {
//             name: name.to_string(),
//             path: path.to_string(),
//             default,
//             // value: None,
//             // annotations: vec![],
//         }
//     }
// }

impl Property {
    pub fn value(&self) -> Result<&Value, &str> {
        match self {
            Property::Entry { default, value } => {
                let value = value.as_ref().unwrap_or(default);
                match value {
                    // If this is a routine, call the routine to return the value
                    // @todo: Allow for caching and #[run_once]
                    Value::Routine(routine) => {
                        // @todo: implement all this using the virtual machine somehow
                        // @todo: handle errors
                        Ok(&*routine.for_now)
                    }
                    // Otherwise, just return the value itself
                    _ => Ok(value),
                }
            }
            Property::Nested { .. } => Err("Cannot call value() on a nested Configuration"),
        }
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), &str> {
        match self {
            Property::Entry {
                default: _,
                value: entry_value,
            } => {
                *entry_value = Some(value);
                Ok(())
            }
            Property::Nested { .. } => Err("Cannot set_value() on a Nested Property"),
        }
    }
}

pub trait Configuration: Any {
    fn set(&mut self, _key: String, _value: Value) {
        // TODO: This needs to actually set this on the values
        // Using :dot.notation.keys
        todo!()
    }

    fn values(&self) -> &ConfigurationEntries;
    fn values_mut(&mut self) -> &mut ConfigurationEntries;
}
