use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Example: Receive a response
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", todos);

    // Example: Send a response
    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "I like trains!".to_owned(),
        completed: false,
    };

    let new_todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);

    Ok(())
}
