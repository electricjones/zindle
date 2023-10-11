use std::collections::HashMap;

use maplit::hashmap;

use crate::{
    configuration::{Configuration, ConfigurationEntries, NestedConfigurations, Property},
    values::Value,
};

/////////// Generated
// Keep the original in comment form
pub struct Sub {
    values: ConfigurationEntries,
    nested: NestedConfigurations,
}

impl Default for Sub {
    fn default() -> Self {
        Self {
            nested: hashmap! {},
            values: hashmap! {
                "a_u64".to_string() =>
                    Property::new(
                        "a_u64",
                        ":sub.a_u64",
                        Value::try_from(Self::default_a_u64()).unwrap(),
                    ),
            // "an_str".to_string() =>  Property::new(
            //     "an_str",
            //     ":sub.an_str",
            //     Value::try_from(Self::default_an_str()).unwrap(),
            // ),
            // "a_f32".to_string() =>  Property::new(
            //     "a_f32",
            //     ":sub.a_f32",
            //     Value::try_from(Self::default_a_f32()).unwrap(),
            // ),
            // "a_routine".to_string() => Property::new(
            //     "a_routine",
            //     ":sub.a_routine",
            //     Value::Routine(Routine::default()),
            // ),

            },
        }
    }
}

impl Configuration for Sub {
    fn values(&self) -> &ConfigurationEntries {
        &self.values
    }

    fn values_mut(&mut self) -> &mut ConfigurationEntries {
        &mut self.values
    }
}

// Generated
impl Sub {
    pub fn default_a_u64() -> u64 {
        200
    }

    pub fn a_u64(&self) -> u64 {
        self.values
            .get("a_u64")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_a_u64_value(&mut self, value: u64) {
        let built_value = Value::try_from(value).unwrap();
        self.values.get_mut("a_u64").unwrap().set_value(built_value);
    }

    pub fn default_an_str() -> &'static str {
        "james"
    }

    pub fn an_str<'a>(&'a self) -> &'a str {
        self.values
            .get("an_str")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_an_str_value(&mut self, value: &str) {
        let built_value = Value::try_from(value).unwrap();
        self.values
            .get_mut("an_str")
            .unwrap()
            .set_value(built_value);
    }

    pub fn default_a_f32() -> f32 {
        500.3001
    }

    pub fn a_f32(&self) -> f32 {
        self.values
            .get("a_f32")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_a_f32_value(&mut self, value: f32) {
        let built_value = Value::try_from(value).unwrap();
        self.values.get_mut("a_f32").unwrap().set_value(built_value);
    }

    pub fn default_a_routine() -> String {
        String::from("the default routine value")
    }

    pub fn a_routine(&self) -> String {
        self.values
            .get("a_routine")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_a_routine_value(&mut self, value: String) {
        let built_value = Value::try_from(value).unwrap();
        self.values
            .get_mut("a_routine")
            .unwrap()
            .set_value(built_value);
    }
}

pub struct Root {
    values: ConfigurationEntries,
    nested: NestedConfigurations,
}

impl Configuration for Root {
    fn values(&self) -> &configuration::ConfigurationEntries {
        &self.values
    }

    fn values_mut(&mut self) -> &mut configuration::ConfigurationEntries {
        &mut self.values
    }
}

impl Default for Root {
    fn default() -> Self {
        Self {
            nested: hashmap! {
                "sub".to_string() => Box::new(Sub::default()) as Box<dyn Configuration>
            },
            values: hashmap! {
                // "an_i32".to_string() =>  Property::new(
                //     "an_i32",
                //     ":an_i32",
                //     Value::try_from(Self::default_an_i32()).unwrap(),
                // ),
                // "a_bool".to_string() => Property::new(
                //     "a_bool",
                //     ":a_bool",
                //     Value::try_from(Self::default_a_bool()).unwrap(),
                // ),
                // "a_string".to_string() => Property::new(
                //     "a_string",
                //     ":a_string",
                //     Value::try_from(Self::default_a_string()).unwrap(),
                // ),
                // "an_array".to_string() => Property::new(
                //     "an_array",
                //     ":an_array",
                //     Value::try_from(Self::default_an_array()).unwrap(),
                // ),
                // "a_mixed_array".to_string() => Property::new(
                //     "a_mixed_array",
                //     ":a_mixed_array",
                //     Value::try_from(Self::default_a_mixed_array()).unwrap(),
                // ),
                // "a_dictionary".to_string() => Property::new(
                //     "a_dictionary",
                //     ":a_dictionary",
                //     Value::try_from(Self::default_a_dictionary()).unwrap(),
                // ),
                // "a_mixed_dictionary".to_string() => Property::new(
                //     "a_mixed_dictionary",
                //     ":a_mixed_dictionary",
                //     Value::try_from(Self::default_a_mixed_dictionary()).unwrap(),
                // ),
                // // "sub".to_string() => Property::default(),
                //
            },
        }
    }
}

// Generated getters
impl Root {
    pub fn default_an_i32() -> i32 {
        28
    }

    pub fn an_i32(&self) -> i32 {
        self.values
            .get("an_i32")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_an_i32_value(&mut self, value: i32) {
        let built_value = Value::try_from(value).unwrap();
        self.values
            .get_mut("an_i32")
            .unwrap()
            .set_value(built_value);
    }

    pub fn default_a_bool() -> bool {
        false
    }

    pub fn a_bool(&self) -> bool {
        self.values
            .get("a_bool")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_a_bool_value(&mut self, value: bool) {
        let built_value = Value::try_from(value).unwrap();
        self.values
            .get_mut("a_bool")
            .unwrap()
            .set_value(built_value);
    }

    pub fn default_a_string() -> String {
        String::from("michael")
    }

    pub fn a_string(&self) -> String {
        self.values
            .get("a_string")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_a_string_value(&mut self, value: String) {
        let built_value = Value::try_from(value).unwrap();
        self.values
            .get_mut("a_string")
            .unwrap()
            .set_value(built_value);
    }

    pub fn default_an_array() -> Vec<bool> {
        vec![true, false, false, true, false]
    }

    pub fn an_array(&self) -> Vec<bool> {
        self.values
            .get("an_array")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_an_array_value(&mut self, value: Vec<bool>) {
        let built_value = Value::try_from(value).unwrap();
        self.values
            .get_mut("an_array")
            .unwrap()
            .set_value(built_value);
    }

    pub fn default_a_mixed_array() -> Vec<Value> {
        vec![
            Value::try_from(76).unwrap(),
            Value::try_from(true).unwrap(),
            Value::try_from(12.8).unwrap(),
        ]
    }

    pub fn a_mixed_array(&self) -> Vec<Value> {
        self.values
            .get("a_mixed_array")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_a_mixed_array_value(&mut self, value: Vec<Value>) {
        let built_value = Value::try_from(value).unwrap();
        self.values
            .get_mut("a_mixed_array")
            .unwrap()
            .set_value(built_value);
    }

    pub fn default_a_dictionary() -> HashMap<String, f32> {
        HashMap::from([("One".to_string(), 20.1), ("Two".to_string(), 30.2)])
    }

    pub fn a_dictionary(&self) -> HashMap<String, f32> {
        self.values
            .get("a_dictionary")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_a_dictionary_value(&mut self, value: HashMap<String, f32>) {
        let built_value = Value::try_from(value).unwrap();
        self.values
            .get_mut("a_dictionary")
            .unwrap()
            .set_value(built_value);
    }

    pub fn default_a_mixed_dictionary() -> HashMap<String, Value> {
        HashMap::from([
            ("One".to_string(), Value::try_from(20000.001).unwrap()),
            ("Two".to_string(), Value::try_from(true).unwrap()),
        ])
    }

    pub fn a_mixed_dictionary(&self) -> HashMap<String, Value> {
        self.values
            .get("a_mixed_dictionary")
            .unwrap() // This is safe to unwrap, its guarenteed
            .value()
            .try_into()
            .unwrap()
    }

    pub fn set_a_mixed_dictionary_value(&mut self, value: HashMap<String, Value>) {
        let built_value = Value::try_from(value).unwrap();
        self.values
            .get_mut("a_mixed_dictionary")
            .unwrap()
            .set_value(built_value);
    }

    pub fn sub(&self) -> &Sub {
        todo!()
    }

    pub fn sub_mut(&mut self) -> &mut Sub {
        todo!()
    }
}
