use crate::values::Value;
use std::collections::HashMap;

pub mod collection;

pub type ConfigurationEntries = HashMap<String, Property>;
pub type NestedConfigurations = HashMap<String, Box<dyn Configuration>>;

// TODO: This is a temporary solution
// I do this so that I can keep a Sub and a Value in the same hashmap
// Ultimately, I don't want to have an enum inside an enum inside an enum

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    default: Value,
    // validations: Vec<Annotation>,
    name: String,
    path: String, // Fully qualified namespace
}

impl Property {
    pub fn new(name: &str, path: &str, default: Value) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            default,
            // value: None,
            // annotations: vec![],
        }
    }
}

impl Property {
    pub fn value(&self) -> &Value {
        todo!()
        //     // self.value.as_ref().unwrap_or(&self.default)
        //
        //     // If there is already a value, use that value
        //     // If there is no value, and **there is a routine**, run the routine
        //     // If there is a routine, and it errors, use the default
        //     // If there is no value and no routine, use the default
        //
        //     let value = self.value.as_ref().unwrap_or(&self.default);
        //     match value {
        //         // If this is a routine, call the routine to return the value
        //         // @todo: Allow for caching and #[run_once]
        //         Value::Routine(routine) => {
        //             // @todo: implement all this using the virtual machine somehow
        //             // @todo: handle errors
        //             &*routine.for_now
        //         }
        //         // Otherwise, just return the value itself
        //         _ => value,
        //     }
    }
    //
    pub fn set_value(&mut self, value: Value) {
        todo!();
        //     self.value = Some(value)
    }
}

pub trait Configuration {
    fn set(&mut self, _key: String, _value: Value) {
        // TODO: This needs to actually set this on the values
        // Using :dot.notation.keys
        todo!()
    }

    fn values(&self) -> &ConfigurationEntries;
    fn values_mut(&mut self) -> &mut ConfigurationEntries;
}
