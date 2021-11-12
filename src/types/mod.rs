use std::fmt::{Debug, Display, Error, Formatter};

pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>
}

pub struct QuestionId(pub String);

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id, title, content, tags
        }
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write![f, "{}, title: {}, content: {}, tags: {:?}", self.id, self.title, self.content, self.tags]
    }
}

impl Display for QuestionId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write![f, "id: {}", self.0]
    }
}

impl Debug for Question {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write![f, "{:?}", self.tags]
    }
}