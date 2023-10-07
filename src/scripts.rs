pub mod collection;
pub mod content;
pub mod version;

use self::content::ScriptContent;
use self::version::ScriptVersion;
use std::io::Result as IoResult;

pub type ScriptFilePath = String;
pub type ScriptId = String;
pub type ScriptPath = String;

#[derive(Debug)]
struct ScriptMeta {
    version: ScriptVersion,
    path: ScriptPath,
    // name: String,
}

#[derive(Debug)]
pub struct Script {
    meta: ScriptMeta,
    content: ScriptContent,
}

// TODO: Maybe a way to eager load scripts
impl Script {
    pub fn from_raw(content: String, _name: String) -> Self {
        let version = ScriptVersion::from_contents(&content);
        let meta = ScriptMeta {
            // name,
            path: format!("raw:/{}", version),
            version,
        };

        let content = ScriptContent::Raw(content);

        Self { meta, content }
    }

    pub fn content(&mut self) -> IoResult<&str> {
        self.content.fetch()
    }
}
