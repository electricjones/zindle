use crate::{processor::ProcessedScriptMap, values::Value};
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
    fn set(&mut self, key: &str, value: Value) -> Result<(), String> {
        set_nested(&mut self.values_mut(), key, value)
    }

    fn values(&self) -> &ConfigurationEntries;
    fn values_mut(&mut self) -> &mut ConfigurationEntries;

    fn merge_script_map(&mut self, map: ProcessedScriptMap) -> Result<(), String> {
        for (k, v) in map {
            self.set(&k, v)?
        }

        Ok(())
    }
}

fn set_nested(
    fields: &mut HashMap<String, Property>,
    key: &str,
    value: Value,
) -> Result<(), String> {
    let mut parts = key.splitn(2, '.');
    if let Some(first) = parts.next() {
        match fields.get_mut(first) {
            Some(Property::Nested { value: nested }) => {
                if let Some(rest) = parts.next() {
                    nested.set(rest, value)?;
                } else {
                    return Err(format!("Invalid key: {}", key));
                }
            }
            Some(Property::Entry { .. }) => {
                if parts.next().is_none() {
                    let v = fields.get_mut(first).unwrap();
                    v.set_value(value).unwrap();
                } else {
                    return Err(format!("Invalid key: {}", key));
                }
            }
            None => {
                return Err(format!("Invalid key: {}", key));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use crate::values::Value;

    use super::{Configuration, ConfigurationEntries, Property};

    #[derive(Default)]
    struct MockedConfig {
        pub entries: ConfigurationEntries,
    }

    impl Configuration for MockedConfig {
        fn values(&self) -> &super::ConfigurationEntries {
            &self.entries
        }

        fn values_mut(&mut self) -> &mut super::ConfigurationEntries {
            &mut self.entries
        }
    }

    // #[test]
    // fn it_merges_a_map() {
    //     let mut config = MockedConfig {
    //         entries: hashmap! {
    //             "testing".to_string() => Property::Entry { default: Value::Int(12), value: None },
    //         },
    //     };
    //
    //     let map = hashmap! {
    //         "testing".to_string() => Value::Int(200),
    //     };
    //
    //     config.merge_script_map(map).unwrap();
    //
    //     let actual: i128 = config
    //         .entries
    //         .get("testing")
    //         .unwrap()
    //         .clone()
    //         .value()
    //         .unwrap() as i32;
    //
    //     assert_eq!(200, actual);
    // }
}
