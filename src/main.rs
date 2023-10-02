use configuration::collection::Property;
use values::{Value, ValueError};

pub mod configuration;
pub mod runtime;
pub mod values;

pub struct MySubConfig {
    a_u64: Property,
    // an_str: Property,
    a_f32: Property,
}

impl Default for MySubConfig {
    fn default() -> Self {
        Self {
            a_u64: Property::new(
                "a_u64",
                ":sub.a_u64",
                Value::try_from(Self::default_a_u64()).unwrap(),
            ),
            // an_str: Property::new(
            //     "an_str",
            //     ":an_str",
            //     Value::try_from(Self::default_an_str()).unwrap(),
            // ),
            a_f32: Property::new(
                "a_f32",
                ":sub.a_f32",
                Value::try_from(Self::default_a_f32()).unwrap(),
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

    // pub fn default_a_str() -> &'a str {
    //     false
    // }
    //
    // pub fn a_bool(&self) -> bool {
    //     self.a_bool.value().try_into().unwrap()
    // }
    //
    // pub fn set_a_bool_value(&mut self, value: bool) {
    //     let built_value = Value::try_from(value).unwrap();
    //     self.a_bool.set_value(built_value);
    // }

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

    pub fn sub(&self) -> &MySubConfig {
        &self.sub
    }

    pub fn sub_mut(&mut self) -> &mut MySubConfig {
        &mut self.sub
    }
}

fn main() {
    // @todo: Runtime Builder Pattern
    // let mut runtime = Runtime::new();

    // let config = MyConfig::default();

    // let a = config.a();
}

#[cfg(test)]
mod tests {
    use crate::MyConfig;

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
        config.sub_mut().set_a_f32_value(222.333);

        assert_eq!(111, config.sub().a_u64());
        assert_eq!(222.333, config.sub().a_f32());
    }
}
