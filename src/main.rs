#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

pub mod models;
pub mod controllers;
pub mod response;
pub mod services;

use std::{sync::Arc, collections::HashMap};

use futures::lock::Mutex;
use response::generic_response::GenericResponse;
use models::todo::Todo;
use rocket::State;

use controllers::router::get_routes;

type _TodosState = State<Arc<Mutex<HashMap<String, Todo>>>>;

#[launch]
fn rocket() -> _ {

    let todos: Arc<Mutex<HashMap<String, Todo>>> = Arc::new(Mutex::new(HashMap::new()));

    rocket::build()
        .manage(todos)
        .mount("/api", get_routes())
}


// #[get("/health")]
// async fn health(todos: &TodosState) -> GenericResponse<> {

// }