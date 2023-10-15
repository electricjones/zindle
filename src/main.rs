#![feature(trait_upcasting)]
extern crate derive_builder;

use generated::Root;
use runtime::{Runtime, RuntimeBuilder};
use scripts::Script;

pub mod configuration;
pub mod generated;
pub mod processor;
pub mod runtime;
pub mod scripts;
mod utilities;
pub mod values;

// #[derive(Configuration)]
// pub struct Sub {
//     a_u64: u64,
//     an_str: &'static str,
//     a_f32: f32,
//     a_routine: i32,
// }
//
// // #[derive(Configuration)]
// pub struct Root {
//     // #[configurable]
//     // #[default(28)]
//     an_i32: i32,
//     a_bool: bool,
//     a_string: String,
//     an_array: Vec<bool>,
//     a_mixed_array: Vec<Value>,
//     a_dictionary: HashMap<String, bool>,
//     a_mixed_dictionary: HashMap<String, Value>,
//
//     sub: Sub,
// }

fn main() {
    let config = Root::default();
    let runtime = RuntimeBuilder::default()
        .config(config)
        .add_script(Script::from_raw(
            String::from("Test script contents"),
            "my-test-script".into(),
        ));

    // Many ways to set attach scripts
    // runtime.scripts().add_directory("/path/to/scripts");
    // runtime.scripts().attach(Script::new());
    // runtime.scripts().add_raw("name", "script");
    // runtime.scripts().add_file("/path/to/whatever.zindle");
    //
    // Configure settings
    // runtime.config().sub_mut().set_a_f32_value(20.22);

    // Attach context
    // runtime.add_context("name", value, true);
    //
    // Register event hooks
    // runtime.add_event("EventName", []);
    //
    // runtime.start();
    // # Process scripts
    //  - Parse the script files
    //  - Build HashMaps with the key-value pairs
    //

    let mut runtime: Runtime<Root> = runtime.build().unwrap();

    // Now, we can get the party started
    runtime.start().unwrap();

    // compile, etc
    // runtime.start(); // Startup vm and do eager loading
    //
    // // Finally, I use it like I nomrally would
    // runtime.config.sub().a_string();
    // // or, this just passes down
    // runtime.get("a_routine", "override_default instead of default on config");
    // runtime.get(":sub.a_routine", "same override");

    // Just to make sure this works
    let value = runtime.config().sub().a_routine();
    println!("{:?}", value);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{generated::Root, values::Value};

    #[test]
    fn it_uses_the_default_values() {
        let config = Root::default();

        // Use the default value
        assert_eq!(28, config.an_i32());
        assert_eq!(false, config.a_bool());
        assert_eq!("michael".to_string(), config.a_string());
    }

    #[test]
    fn it_uses_statically_set_values() {
        let mut config = Root::default();

        config.set_an_i32_value(100);
        config.set_a_bool_value(true);
        config.set_a_string_value("wilson".to_string());

        assert_eq!(100, config.an_i32());
        assert_eq!(true, config.a_bool());
        assert_eq!("wilson".to_string(), config.a_string());
    }

    #[test]
    fn it_reads_defaults_from_the_sub_config() {
        let config = Root::default();

        assert_eq!(28, config.an_i32());
        assert_eq!(false, config.a_bool());
        assert_eq!("michael".to_string(), config.a_string());

        assert_eq!(200, config.sub().a_u64());
        assert_eq!("james", config.sub().an_str());
        assert_eq!(500.3001, config.sub().a_f32());
    }

    #[test]
    fn it_reads_statically_set_values_on_the_sub_config() {
        let mut config = Root::default();

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
        let config = Root::default();

        let expected = Root::default_an_array();
        assert_eq!(expected, config.an_array());
    }

    #[test]
    fn it_uses_statically_set_values_for_arrays() {
        let mut config = Root::default();

        config.set_an_array_value(vec![false, true]);

        assert_eq!(vec![false, true], config.an_array());
    }

    #[test]
    fn it_uses_the_default_values_for_dictionaries() {
        let config = Root::default();

        let expected = Root::default_a_dictionary();
        assert_eq!(expected, config.a_dictionary());
    }

    #[test]
    fn it_uses_statically_set_values_for_dictionaries() {
        let mut config = Root::default();

        let expected = HashMap::from([("a".to_string(), 12.1), ("b".to_string(), 13.2)]);

        config.set_a_dictionary_value(expected.clone());

        assert_eq!(expected, config.a_dictionary());
    }

    #[test]
    fn it_uses_the_default_values_for_mixed_arrays() {
        let config = Root::default();

        let expected = Root::default_a_mixed_array();
        assert_eq!(expected, config.a_mixed_array());
    }

    #[test]
    fn it_uses_statically_set_values_for_mixed_arrays() {
        let mut config = Root::default();

        let expected = vec![
            Value::try_from("oreo".to_string()).unwrap(),
            Value::try_from(789).unwrap(),
        ];

        config.set_a_mixed_array_value(expected.clone());

        assert_eq!(expected, config.a_mixed_array());
    }

    #[test]
    fn it_uses_the_default_values_for_mixed_dictionaries() {
        let config = Root::default();

        let expected = Root::default_a_mixed_dictionary();
        assert_eq!(expected, config.a_mixed_dictionary());
    }

    #[test]
    fn it_uses_statically_set_values_for_mixed_dictionaries() {
        let mut config = Root::default();

        let expected = HashMap::from([
            ("a".to_string(), Value::try_from(true).unwrap()),
            ("b".to_string(), Value::try_from(13.2).unwrap()),
        ]);

        config.set_a_mixed_dictionary_value(expected.clone());

        assert_eq!(expected, config.a_mixed_dictionary());
    }

    #[test]
    fn it_reads_routine_executed_values() {
        let config = Root::default();

        assert_eq!(
            "this is a temporary value".to_string(),
            config.sub().a_routine()
        );
    }

    // TODO: lots more routine tests
}
