use std::collections::HashMap;

use configuration::collection::{Property, Routine};
use values::Value;

pub mod configuration;
pub mod runtime;
pub mod values;

pub struct MySubConfig {
    a_u64: Property,
    an_str: Property,
    a_f32: Property,
    a_routine: Property,
}

impl Default for MySubConfig {
    fn default() -> Self {
        Self {
            a_u64: Property::new(
                "a_u64",
                ":sub.a_u64",
                Value::try_from(Self::default_a_u64()).unwrap(),
            ),
            an_str: Property::new(
                "an_str",
                ":sub.an_str",
                Value::try_from(Self::default_an_str()).unwrap(),
            ),
            a_f32: Property::new(
                "a_f32",
                ":sub.a_f32",
                Value::try_from(Self::default_a_f32()).unwrap(),
            ),
            a_routine: Property::new(
                "a_routine",
                ":sub.a_routine",
                Value::Routine(Routine::default()),
            ),
        }
    }
}

// Generated
impl MySubConfig {
    pub fn default_a_u64() -> u64 {
        200
    }

    pub fn a_u64(&self) -> u64 {
        self.a_u64.value().try_into().unwrap()
    }

    pub fn set_a_u64_value(&mut self, value: u64) {
        let built_value = Value::try_from(value).unwrap();
        self.a_u64.set_value(built_value);
    }

    pub fn default_an_str() -> &'static str {
        "james"
    }

    pub fn an_str<'a>(&'a self) -> &'a str {
        self.an_str.value().try_into().unwrap()
    }

    pub fn set_an_str_value(&mut self, value: &str) {
        let built_value = Value::try_from(value).unwrap();
        self.an_str.set_value(built_value);
    }

    pub fn default_a_f32() -> f32 {
        500.3001
    }

    pub fn a_f32(&self) -> f32 {
        self.a_f32.value().try_into().unwrap()
    }

    pub fn set_a_f32_value(&mut self, value: f32) {
        let built_value = Value::try_from(value).unwrap();
        self.a_f32.set_value(built_value);
    }

    pub fn default_a_routine() -> String {
        String::from("the default routine value")
    }

    pub fn a_routine(&self) -> String {
        self.a_routine.value().try_into().unwrap()
    }

    pub fn set_a_routine_value(&mut self, value: String) {
        let built_value = Value::try_from(value).unwrap();
        self.a_routine.set_value(built_value);
    }
}

// #[derive(Configuration)]
pub struct MyConfig {
    // #[configurable]
    // #[default(28)]
    // a: i32,
    an_i32: Property,

    // #[configurable]
    // #[default(false)]
    // b: bool,
    a_bool: Property,

    // #[configurable]
    // c: String,
    // with no default_c() you must implement it
    // todo: Allow them to simply implement Default and use those values in building this
    a_string: Property,

    // I don't think that slices would really work here. Stick to vectors
    an_array: Property,
    a_mixed_array: Property,

    a_dictionary: Property,
    a_mixed_dictionary: Property,

    sub: MySubConfig,
}

// Generated by derive
impl Default for MyConfig {
    fn default() -> Self {
        Self {
            an_i32: Property::new(
                "an_i32",
                ":an_i32",
                Value::try_from(Self::default_an_i32()).unwrap(),
            ),
            a_bool: Property::new(
                "a_bool",
                ":a_bool",
                Value::try_from(Self::default_a_bool()).unwrap(),
            ),
            a_string: Property::new(
                "a_string",
                ":a_string",
                Value::try_from(Self::default_a_string()).unwrap(),
            ),
            an_array: Property::new(
                "an_array",
                ":an_array",
                Value::try_from(Self::default_an_array()).unwrap(),
            ),
            a_mixed_array: Property::new(
                "a_mixed_array",
                ":a_mixed_array",
                Value::try_from(Self::default_a_mixed_array()).unwrap(),
            ),
            a_dictionary: Property::new(
                "a_dictionary",
                ":a_dictionary",
                Value::try_from(Self::default_a_dictionary()).unwrap(),
            ),
            a_mixed_dictionary: Property::new(
                "a_mixed_dictionary",
                ":a_mixed_dictionary",
                Value::try_from(Self::default_a_mixed_dictionary()).unwrap(),
            ),

            sub: MySubConfig::default(),
        }
    }
}

// Generated
impl MyConfig {
    pub fn default_an_i32() -> i32 {
        28
    }

    pub fn an_i32(&self) -> i32 {
        self.an_i32.value().try_into().unwrap()
    }

    pub fn set_an_i32_value(&mut self, value: i32) {
        let built_value = Value::try_from(value).unwrap();
        self.an_i32.set_value(built_value);
    }

    pub fn default_a_bool() -> bool {
        false
    }

    pub fn a_bool(&self) -> bool {
        self.a_bool.value().try_into().unwrap()
    }

    pub fn set_a_bool_value(&mut self, value: bool) {
        let built_value = Value::try_from(value).unwrap();
        self.a_bool.set_value(built_value);
    }

    pub fn default_a_string() -> String {
        String::from("michael")
    }

    pub fn a_string(&self) -> String {
        self.a_string.value().try_into().unwrap()
    }

    pub fn set_a_string_value(&mut self, value: String) {
        let built_value = Value::try_from(value).unwrap();
        self.a_string.set_value(built_value);
    }

    pub fn default_an_array() -> Vec<bool> {
        vec![true, false, false, true, false]
    }

    pub fn an_array(&self) -> Vec<bool> {
        self.an_array.value().try_into().unwrap()
    }

    pub fn set_an_array_value(&mut self, value: Vec<bool>) {
        let built_value = Value::try_from(value).unwrap();
        self.an_array.set_value(built_value);
    }

    pub fn default_a_mixed_array() -> Vec<Value> {
        vec![
            Value::try_from(76).unwrap(),
            Value::try_from(true).unwrap(),
            Value::try_from(12.8).unwrap(),
        ]
    }

    pub fn a_mixed_array(&self) -> Vec<Value> {
        self.a_mixed_array.value().try_into().unwrap()
    }

    pub fn set_a_mixed_array_value(&mut self, value: Vec<Value>) {
        let built_value = Value::try_from(value).unwrap();
        self.a_mixed_array.set_value(built_value);
    }

    pub fn default_a_dictionary() -> HashMap<String, f32> {
        HashMap::from([("One".to_string(), 20.1), ("Two".to_string(), 30.2)])
    }

    pub fn a_dictionary(&self) -> HashMap<String, f32> {
        self.a_dictionary.value().try_into().unwrap()
    }

    pub fn set_a_dictionary_value(&mut self, value: HashMap<String, f32>) {
        let built_value = Value::try_from(value).unwrap();
        self.a_dictionary.set_value(built_value);
    }

    pub fn default_a_mixed_dictionary() -> HashMap<String, Value> {
        HashMap::from([
            ("One".to_string(), Value::try_from(20000.001).unwrap()),
            ("Two".to_string(), Value::try_from(true).unwrap()),
        ])
    }

    pub fn a_mixed_dictionary(&self) -> HashMap<String, Value> {
        self.a_mixed_dictionary.value().try_into().unwrap()
    }

    pub fn set_a_mixed_dictionary_value(&mut self, value: HashMap<String, Value>) {
        let built_value = Value::try_from(value).unwrap();
        self.a_mixed_dictionary.set_value(built_value);
    }

    pub fn sub(&self) -> &MySubConfig {
        &self.sub
    }

    pub fn sub_mut(&mut self) -> &mut MySubConfig {
        &mut self.sub
    }
}

fn main() {
    let val = vec![true, false, false, true, false];
    let value: Value = Value::try_from(val).unwrap();

    println!("{:?}", value);

    // @todo: Runtime Builder Pattern
    // let mut runtime = Runtime::new();

    // let config = MyConfig::default();

    // let a = config.a();
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{configuration::collection::Routine, values::Value, MyConfig};

    #[test]
    fn it_uses_the_default_values() {
        let config = MyConfig::default();

        // Use the default value
        assert_eq!(28, config.an_i32());
        assert_eq!(false, config.a_bool());
        assert_eq!("michael".to_string(), config.a_string());
    }

    #[test]
    fn it_uses_statically_set_values() {
        let mut config = MyConfig::default();

        config.set_an_i32_value(100);
        config.set_a_bool_value(true);
        config.set_a_string_value("wilson".to_string());

        assert_eq!(100, config.an_i32());
        assert_eq!(true, config.a_bool());
        assert_eq!("wilson".to_string(), config.a_string());
    }

    #[test]
    fn it_reads_defaults_from_the_sub_config() {
        let config = MyConfig::default();

        assert_eq!(28, config.an_i32());
        assert_eq!(false, config.a_bool());
        assert_eq!("michael".to_string(), config.a_string());

        assert_eq!(200, config.sub().a_u64());
        assert_eq!("james", config.sub().an_str());
        assert_eq!(500.3001, config.sub().a_f32());
    }

    #[test]
    fn it_reads_statically_set_values_on_the_sub_config() {
        let mut config = MyConfig::default();

        config.set_an_i32_value(100);
        config.set_a_bool_value(true);
        config.set_a_string_value("wilson".to_string());

        assert_eq!(100, config.an_i32());
        assert_eq!(true, config.a_bool());
        assert_eq!("wilson".to_string(), config.a_string());

        config.sub_mut().set_a_u64_value(111);
        config.sub_mut().set_an_str_value("richards");
        config.sub_mut().set_a_f32_value(222.333);

        assert_eq!(111, config.sub().a_u64());
        assert_eq!("richards", config.sub().an_str());
        assert_eq!(222.333, config.sub().a_f32());
    }

    #[test]
    fn it_uses_the_default_values_for_arrays() {
        let config = MyConfig::default();

        let expected = MyConfig::default_an_array();
        assert_eq!(expected, config.an_array());
    }

    #[test]
    fn it_uses_statically_set_values_for_arrays() {
        let mut config = MyConfig::default();

        config.set_an_array_value(vec![false, true]);

        assert_eq!(vec![false, true], config.an_array());
    }

    #[test]
    fn it_uses_the_default_values_for_dictionaries() {
        let config = MyConfig::default();

        let expected = MyConfig::default_a_dictionary();
        assert_eq!(expected, config.a_dictionary());
    }

    #[test]
    fn it_uses_statically_set_values_for_dictionaries() {
        let mut config = MyConfig::default();

        let expected = HashMap::from([("a".to_string(), 12.1), ("b".to_string(), 13.2)]);

        config.set_a_dictionary_value(expected.clone());

        assert_eq!(expected, config.a_dictionary());
    }

    #[test]
    fn it_uses_the_default_values_for_mixed_arrays() {
        let config = MyConfig::default();

        let expected = MyConfig::default_a_mixed_array();
        assert_eq!(expected, config.a_mixed_array());
    }

    #[test]
    fn it_uses_statically_set_values_for_mixed_arrays() {
        let mut config = MyConfig::default();

        let expected = vec![
            Value::try_from("oreo".to_string()).unwrap(),
            Value::try_from(789).unwrap(),
        ];

        config.set_a_mixed_array_value(expected.clone());

        assert_eq!(expected, config.a_mixed_array());
    }

    #[test]
    fn it_uses_the_default_values_for_mixed_dictionaries() {
        let config = MyConfig::default();

        let expected = MyConfig::default_a_mixed_dictionary();
        assert_eq!(expected, config.a_mixed_dictionary());
    }

    #[test]
    fn it_uses_statically_set_values_for_mixed_dictionaries() {
        let mut config = MyConfig::default();

        let expected = HashMap::from([
            ("a".to_string(), Value::try_from(true).unwrap()),
            ("b".to_string(), Value::try_from(13.2).unwrap()),
        ]);

        config.set_a_mixed_dictionary_value(expected.clone());

        assert_eq!(expected, config.a_mixed_dictionary());
    }

    #[test]
    fn it_reads_routine_executed_values() {
        let config = MyConfig::default();

        assert_eq!(
            "this is a temporary value".to_string(),
            config.sub().a_routine()
        );
    }

    // TODO: lots more routine tests
}
