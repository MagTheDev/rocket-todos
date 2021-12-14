use std::{sync::Arc, collections::HashMap};

use futures::lock::Mutex;
use rocket::State;

use crate::models::todo::Todo;

type TodosState = State<Arc<Mutex<HashMap<String, Todo>>>>;


pub async fn get_todos(todos: &TodosState) -> Vec<Todo>{
    let todos = todos.lock().await;
    let todos_out: Vec<Todo> = (*todos).values().cloned().collect();
    todos_out
}

pub async fn create_todo(todos: &TodosState, todo: Todo) -> bool {
    let mut todos = todos.lock().await;
    (*todos).insert(todo.clone().uuid, todo);
    true
}

pub async fn edit_todo(todos: &TodosState, todo: Todo) -> Option<Todo> {

    let mut todos = todos.lock().await;

    let old_todo = (*todos).get_mut(&todo.uuid)?;

    old_todo.replace(todo);
    
    Some(old_todo.clone())
}

pub async fn finish_todo(todos: &TodosState, uuid: String) -> Option<Todo> {
    let mut todos = todos.lock().await;
    let todo = (*todos).get_mut(&uuid)?;
    todo.set_is_done(true);
    Some(todo.clone())
}