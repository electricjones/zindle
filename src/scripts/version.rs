use std::fmt::Display;

#[derive(Debug)]
pub struct ScriptVersion {
    version: String,
    short: String,
}

impl ScriptVersion {
    pub fn from_contents(contents: &String) -> Self {
        let version = format!("{:x}", md5::compute(contents));
        let short = version.chars().take(7).collect();

        Self { version, short }
    }

    pub fn short(&self) -> &String {
        &self.short
    }
}

impl Display for ScriptVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

impl From<ScriptVersion> for String {
    fn from(value: ScriptVersion) -> Self {
        value.version
    }
}
