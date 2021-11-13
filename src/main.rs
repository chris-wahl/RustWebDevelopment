use warp::Filter;

use crate::types::{Question, QuestionId};

mod types;
mod routes;

#[tokio::main]
async fn main() {
    let _hello = warp::get()
        .map(|| format!("Hello, World!"));
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(routes::get_questions)
        .recover(routes::return_error);


    warp::serve(get_items)
        .run(([127, 0, 0, 1], 1337))
        .await;

    println!["Running"];
}