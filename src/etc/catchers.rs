use rocket::{http::Status, serde::json::Json, Request};

use crate::model::wrapper::ResponseWrapper;

#[catch(401)]
pub fn unauthorized_error(_: &Request) -> Json<ResponseWrapper<String>> {
    Json(ResponseWrapper::message(
        Status::Unauthorized,
        String::from("Unauthorized access."),
    ))
}

// 400 Bad Request
#[catch(400)]
pub fn bad_request_error(_: &Request) -> Json<ResponseWrapper<String>> {
    Json(ResponseWrapper::message(
        Status::BadRequest,
        String::from("Bad Request."),
    ))
}

// 404 Not Found
#[catch(404)]
pub fn not_found_error(_: &Request) -> Json<ResponseWrapper<String>> {
    Json(ResponseWrapper::message(
        Status::NotFound,
        String::from("Not Found."),
    ))
}

// 422 Unprocessable Entity
#[catch(422)]
pub fn unprocessable_entity_error(_: &Request) -> Json<ResponseWrapper<String>> {
    Json(ResponseWrapper::message(
        Status::UnprocessableEntity,
        String::from("Unprocessable Entity."),
    ))
}

// 409 Conflict
#[catch(409)]
pub fn conflict_error(_: &Request) -> Json<ResponseWrapper<String>> {
    Json(ResponseWrapper::message(
        Status::Conflict,
        String::from("Conflict."),
    ))
}

// 500 Internal Server Error
#[catch(500)]
pub fn internal_server_error(_: &Request) -> Json<ResponseWrapper<String>> {
    Json(ResponseWrapper::message(
        Status::InternalServerError,
        String::from("Internal Server Error."),
    ))
}
