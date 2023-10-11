use super::{Script, ScriptId};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ScriptsCollection {
    scripts: HashMap<ScriptId, Script>,
}

impl ScriptsCollection {
    pub fn add(&mut self, script: Script, _priority: Option<u8>) {
        let script_id = format!("{}@{}", script.meta.path, script.meta.version);
        self.scripts.insert(script_id, script);
    }

    pub fn is_empty(&self) -> bool {
        self.scripts.is_empty()
    }
}

// TODO: Implement iter from ChatGPT
