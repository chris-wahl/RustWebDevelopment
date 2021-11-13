use std::{io::ErrorKind, str::FromStr};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Question {
    pub(crate) id: QuestionId,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct QuestionId(pub(crate) String);

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(std::io::Error::new(ErrorKind::InvalidInput, "No id provided"))
        }
    }
}