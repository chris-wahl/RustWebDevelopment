use crate::types::{Question, QuestionId};

mod types;


fn main() {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(Vec::from(["faq".to_string()]))
    );
    println!["{}", question];
}
