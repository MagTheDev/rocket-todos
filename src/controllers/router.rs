use rocket::Route;

use crate::controllers::todo::*;


pub fn get_routes() -> Vec<Route> {

    return routes![
        create_todo, get_todos, update_todo, delete_todo, mark_as_done,
    ]

}