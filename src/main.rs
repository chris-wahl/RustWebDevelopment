use std::str::FromStr;
use crate::types::{Question, QuestionId};

mod types;


fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(Vec::from(["faq".to_string()]))
    );
    println!["{:?}", question];
}
