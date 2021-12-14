use std::io::Cursor;

use serde::Serialize;
use rocket::{response::Responder, http::{Status, ContentType}};

#[derive(Serialize)]
pub struct GenericResponse<T> {

    pub response: T,
    pub status_code: u16,
    
}

impl<'r, T> Responder<'r, 'static> for GenericResponse<T> where T: Serialize {
    fn respond_to(self, _request: &rocket::Request) -> rocket::response::Result<'static> {
        let json = match serde_json::to_string_pretty(&self) {

            Ok(res) => res,
            Err(_) => return Result::Err(Status::InternalServerError)

        };

        let response = rocket::Response::build()
            .sized_body(json.len(),Cursor::new(json))
            .header(ContentType::new("application", "json"))
            .status(Status::from_code(self.status_code).unwrap_or(Status::ImATeapot))
            .finalize();

        Ok(response)
    }
}

impl<T> GenericResponse<T> where T: Serialize {

    pub fn new(response: T, status_code: u16) -> Self {
        GenericResponse {
            response,
            status_code
        }
    }
}
