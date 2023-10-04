use std::{collections::HashMap, fmt::Display};
pub type ScriptContents = String;
pub type ScriptFilePath = String;

// TODO: Lazy load script files or eager load?
// TODO: How do I want to store the contents of the files?
// TODO: When to validate scripts? (probably in compile time, not Script creation)
// TODO: Will need precedent rules

#[derive(Debug)]
pub enum ScriptPath {
    File(ScriptFilePath, Option<ScriptContents>),
    Raw(ScriptContents),
}

#[derive(Debug)]
pub struct Script {
    path: ScriptPath,
    version: ScriptVersion,
}

impl Script {
    pub fn path(&self) -> &ScriptPath {
        &self.path
    }

    pub fn from_raw(contents: String) -> Self {
        Self {
            version: ScriptVersion::from_contents(&contents),
            path: ScriptPath::Raw(contents),
        }
    }
}

pub type ScriptId = String;

const SCRIPT_VERSION_MAX_LENGTH: usize = 10;
#[derive(Debug)]
pub struct ScriptVersion {
    version: [char; SCRIPT_VERSION_MAX_LENGTH],
}

impl Display for ScriptVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &c in &self.version {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl ScriptVersion {
    fn from_contents(contents: &String) -> Self {
        let sub_string: String = contents.chars().take(SCRIPT_VERSION_MAX_LENGTH).collect();

        let mut version = [' '; 10]; // initialize array with spaces
        for (i, c) in sub_string.chars().enumerate() {
            version[i] = c.clone();
        }

        Self { version }
    }
}

impl From<ScriptVersion> for String {
    fn from(value: ScriptVersion) -> Self {
        value.version.iter().collect()
    }
}

#[derive(Debug, Default)]
pub struct ScriptsCollection {
    scripts: HashMap<ScriptId, Script>,
}

impl ScriptsCollection {
    pub fn add(&mut self, script: Script, _priority: Option<u8>) {
        let script_id = match script.path() {
            ScriptPath::File(path, _) => format!("file://{path}"),
            ScriptPath::Raw(_) => format!("raw://{}", script.version),
        };

        self.scripts.insert(script_id, script);
    }

    pub fn is_empty(&self) -> bool {
        self.scripts.is_empty()
    }
}
