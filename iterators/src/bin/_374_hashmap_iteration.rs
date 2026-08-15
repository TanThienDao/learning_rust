
use std::collections::HashMap;

fn main() {
    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);

    for (todo, completion_status) in todos.iter_mut() {
        *completion_status = true; // Mark all tasks as completed
        println!("Tasks: {}, Completion status: {}", todo, completion_status);
    }
    println!("Total Total Tasks: {:#?}", todos);

    for (_, completion_status) in todos.iter_mut() { // dont intent to do anything with the key, so we use _ to ignore it
        *completion_status = false; // Mark all tasks as not completed
    }

    println!("Total Completion Tasks: {:#?}", todos);
}