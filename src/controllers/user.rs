use mongodb::{bson::doc, sync::Collection};
use rocket::{http::Status, serde::json::Json, State};
use serde_json::json;

use crate::{
    constants::strings::USERS,
    database::mongo::MongoDBState,
    guards::jwt_token::JwtToken,
    model::{
        user::{ChangePassword, Login, User},
        wrapper::ResponseWrapper,
    },
    services::generic::Generic,
};
#[post("/change-password", format = "json", data = "<password>")]
pub fn change_password(
    password: Json<ChangePassword>,
    db: &State<MongoDBState>,
    middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the users collection
    let db = db.inner().db();
    let collection: Collection<User> = db.collection::<User>(USERS);

    // Change password in MongoDB
    match User::change_password(&middleware.0.sub, password.0, &collection) {
        Ok(user) => match user {
            Some(_) => Json(ResponseWrapper::new(
                Status::Accepted,
                String::from("Password changed successfully"),
            )),
            None => Json(ResponseWrapper::message(
                Status::UnprocessableEntity,
                String::from("Unable to change pasword"),
            )),
        },
        Err(_) => Json(ResponseWrapper::message(
            Status::InternalServerError,
            format!("Failed to change password"),
        )),
    }
}

#[post("/register", format = "json", data = "<user>")]
pub fn register(user: Json<User>, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the users collection
    let db = db.inner().db();
    let collection: Collection<User> = db.collection::<User>(USERS);

    let user = &user.0.hash();
    // Insert the document into MongoDB
    match User::insert_resource(&user, &collection) {
        Ok(_) => json!({ "status": "success", "response": "User Created successfully" }),
        Err(err) => {
            json!({ "status": "failed", "response": format!("Failed to create user , {}",err.to_string()) })
        }
    }
}

#[get("/")]
pub fn get_user(db: &State<MongoDBState>, middleware: JwtToken) -> serde_json::Value {
    // Get the users collection
    let db = db.inner().db();
    let collection: Collection<User> = db.collection::<User>(USERS);

    // Get the document of user into MongoDB
    match User::find_resource_by_id(&middleware.0.sub, &collection) {
        Ok(user) => json!({ "status": "success", "response": user.unwrap() }),
        Err(_) => json!({ "status": "failed", "response": "Failed to get user" }),
    }
}

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Json<Login>, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the users collection
    let db = db.inner().db();
    let collection: Collection<User> = db.collection::<User>(USERS);

    // Get the documents of users into MongoDB
    match User::login(&login, &collection) {
        Ok(token) => {
            if token.is_empty() {
                return json!({ "status": "success", "response": "Invalid Credentails" });
            } else {
                return json!({ "status": "success", "response": { "token":token } });
            }
        }
        Err(err) => json!({ "status": "failed", "response": err.to_string()}),
    }
}

#[put("/", format = "json", data = "<user>")]
pub fn update_user(
    user: Json<User>,
    db: &State<MongoDBState>,
    middleware: JwtToken,
) -> serde_json::Value {
    // Get the users collection
    let db = db.inner().db();
    let collection: Collection<User> = db.collection::<User>(USERS);

    // Update the document into MongoDB
    match User::update_resource(&middleware.0.sub, &user.0, &collection) {
        Ok(_) => json!({ "status": "success", "response": "User updated successfully" }),
        Err(err) => json!({ "status": "failed", "response": err.to_string() }),
    }
}
#[delete("/")]
pub fn delete_user(db: &State<MongoDBState>, middleware: JwtToken) -> serde_json::Value {
    // Get the users collection
    let db = db.inner().db();
    let collection: Collection<User> = db.collection::<User>(USERS);

    // Delete the document into MongoDB
    match User::delete_resource(&middleware.0.sub, &collection) {
        Ok(_) => json!({ "status": "success", "response": "User Deleted successfully" }),
        Err(_) => json!({ "status": "failed", "response": "Failed to delete user" }),
    }
}

#[cfg(test)]
mod tests {
    use crate::model::user::User;

    use super::*;

    #[test]
    fn it_should_deserialize_user() {
        let mock = json!({
            "username":"user",
            "email":"me@mail.com",
            "address":[],
            "password":"password"
        });
        let _: User = serde_json::from_str(&mock.to_string()).unwrap();
    }
}
