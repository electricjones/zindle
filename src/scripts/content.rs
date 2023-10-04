use std::fs;
use std::io::Result as IoResult;

pub type ContentType = String;

#[derive(Debug)]
pub enum ScriptContent {
    File {
        file_path: ContentType,
        content: Option<ContentType>,
    },
    Raw(String),
}

impl ScriptContent {
    pub fn fetch(&mut self) -> IoResult<&str> {
        match self {
            ScriptContent::File { file_path, content } => {
                if content.is_none() {
                    *content = Some(fs::read_to_string(file_path)?);
                }
                Ok(content.as_deref().unwrap())
            }
            ScriptContent::Raw(raw_data) => Ok(raw_data),
        }
    }
}
