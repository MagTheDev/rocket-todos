use std::{sync::Arc, collections::HashMap};

use futures::lock::Mutex;
use rocket::{get, State, http::Status};
use crate::{GenericResponse, models::todo::Todo};
use rocket::serde::json::Json;

use crate::services::todo_service;

type TodosState = State<Arc<Mutex<HashMap<String, Todo>>>>;

#[get("/todo")]
pub async fn get_todos(todos: &TodosState) -> GenericResponse<Vec<Todo>> {

    GenericResponse::new(todo_service::get_todos(&todos).await, 200)

}

#[post("/todo", data = "<todo>")]
pub async fn create_todo(todos: &TodosState, mut todo: Json<Todo>) -> GenericResponse<Vec<Todo>> {

    let mut todos = todos.lock().await;
    todo.set_uuid();

    todos.insert(todo.clone().uuid, todo.clone());

    return GenericResponse::new((*todos).clone().values().cloned().collect(), 200);

}

#[put("/todo/<uuid>", data = "<todo>")]
pub async fn update_todo(todos: &TodosState, uuid: String, todo: Json<Todo>) -> Result<GenericResponse<Todo>, Status> {

    let mut todos = todos.lock().await;
    let old_todo = match todos.get_mut(&uuid) {
        Some(todo) => todo,
        None => return Err(Status::NotFound)
    };

    old_todo.replace(todo.clone());

    return Ok(GenericResponse::new(old_todo.clone(), 200));
}

#[delete("/todo/<uuid>")]
pub async fn delete_todo(todos: &TodosState, uuid: String) -> Result<GenericResponse<Todo>, Status> {

    let mut todos = todos.lock().await;
    let todo = match todos.remove(&uuid) {
        Some(todo) => todo,
        None => return Err(Status::NotFound)
    };

    return Ok(GenericResponse::new(todo, 200));
}

#[patch("/todo/<uuid>/done")]
pub async fn mark_as_done(todos: &TodosState, uuid: String) -> Result<GenericResponse<Todo>, Status> {

    match todo_service::finish_todo(&todos, uuid).await {
    Some(s) => return Ok(GenericResponse::new(s, 200)),
    None => return Err(Status::NotFound)
    };
}
