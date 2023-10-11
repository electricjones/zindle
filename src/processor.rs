use std::collections::HashMap;

use maplit::hashmap;

use crate::{
    configuration::collection::Routine, scripts::collection::ScriptsCollection, values::Value,
};

#[derive(Default)]
pub struct Processor {}
impl Processor {
    pub fn create_map_from(_scripts: &ScriptsCollection) -> HashMap<String, Value> {
        // TODO: Much later problem
        hashmap! {
            String::from(":an_i32") => Value::Int(99),
            String::from(":a_routine") => Value::Routine(Routine {
                for_now: Box::new(Value::try_from(1000).unwrap()),
                ..Routine::default()
            }),

            // And one for the sub
            String::from(":sub.a_routine") => Value::Routine(Routine {
                for_now: Box::new(Value::try_from("Another value").unwrap()),
                ..Routine::default()
            }),
        }

        // This must
        //  1. Build the HashMap for each script
        //  2. Compile each Routine
        //  3. Respect precedents (the order the scripts were added)
    }
}
