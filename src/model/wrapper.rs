use std::fmt::Debug;

use rocket::http::Status;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseWrapper<T> {
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl<T> ResponseWrapper<T>
where
    T: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    pub fn new(status: Status, response: T) -> Self {
        Self {
            status,
            response: Some(response),
            message: None,
        }
    }

    pub fn message(status: Status, message: String) -> Self {
        Self {
            status,
            response: None,
            message: Some(message),
        }
    }
}
