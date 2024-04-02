use std::str::FromStr;

use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::Collection,
};
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    constants::strings::{ADDRESSES, USERS},
    database::mongo::MongoDBState,
    guards::jwt_token::JwtToken,
    model::{
        user::User,
        user_address::{UserAddress, UserAddresses},
        wrapper::ResponseWrapper,
    },
    services::generic::Generic,
};

#[post("/many", format = "json", data = "<user_addresses>")]
pub fn create_user_addresses(
    user_addresses: Json<UserAddresses>,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the user_addresses collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);

    // Insert the document into MongoDB
    match UserAddress::insert_multiple_resource(&user_addresses.0, &collection) {
        Ok(_) => Json(ResponseWrapper::new(
            Status::Ok,
            String::from("UserAddresses were created successfully"),
        )),
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Creating user_address , {}",
                err.to_string()
            ),
        )),
    }
}

#[post("/", format = "json", data = "<user_address>")]
pub fn create_user_address(
    user_address: Json<UserAddress>,
    db: &State<MongoDBState>,
    middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the user_addresses collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);
    let user_collection: Collection<User> = db.collection::<User>(USERS);

    // Insert the document into MongoDB
    match UserAddress::insert_resource(&user_address.0, &collection) {
        Ok(result) => {
            let filter = doc! { "_id": ObjectId::from_str(&middleware.0.sub).unwrap() };
            /*
                let address = UserAddress::find_resource_by_object_id(
                    result.inserted_id.as_object_id().unwrap(),
                    &collection,
                )
                .unwrap()
                .unwrap();
            */

            let address_doc = result.inserted_id.as_object_id().unwrap();
            let update = doc! {
                "$set":{
                    "address":address_doc
                }
            };

            match user_collection.find_one_and_update(filter, update, None) {
                Ok(_) => Json(ResponseWrapper::new(
                    Status::Ok,
                    String::from("UserAddress Created successfully"),
                )),
                Err(err) => Json(ResponseWrapper::message(
                    Status::NotFound,
                    format!(
                        "Something Went Wrong While Creating user_address , {}",
                        err.to_string()
                    ),
                )),
            }
        }

        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Creating user_address , {}",
                err.to_string()
            ),
        )),
    }
}

#[get("/<id>")]
pub fn get_user_address<'a>(
    id: &str,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<UserAddress>> {
    // Get the user_addresses collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);

    // Get the document of user_address into MongoDB
    match UserAddress::find_resource_by_id(id, &collection) {
        Ok(db_user_address) => match db_user_address {
            Some(user_address) => Json(ResponseWrapper::new(Status::Ok, user_address)),
            None => Json(ResponseWrapper::message(
                Status::NotFound,
                format!("UserAddress not found with id , {}", id),
            )),
        },
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Getting user_address , {}",
                err.to_string()
            ),
        )),
    }
}

#[get("/")]
pub fn get_user_addresses(
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<UserAddresses>> {
    // Get the user_addresses collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);

    // Get the documents of user_addresses into MongoDB
    match UserAddress::find_resources(&collection) {
        Ok(db_user_addresses) => match db_user_addresses {
            Some(user_addresses) => Json(ResponseWrapper::new(Status::Accepted, user_addresses)),
            None => Json(ResponseWrapper::message(
                Status::NotFound,
                format!("UserAddresses Not Found"),
            )),
        },
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Getting user_address , {}",
                err.to_string()
            ),
        )),
    }
}

#[put("/<id>", format = "json", data = "<user_address>")]
pub fn update_user_address(
    id: &str,
    user_address: Json<UserAddress>,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<UserAddress>> {
    // Get the user_addresses collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);

    // Update the document into MongoDB
    match UserAddress::update_resource(id, &user_address.0, &collection) {
        Ok(db_user_address) => match db_user_address {
            Some(user_address) => Json(ResponseWrapper::new(Status::Ok, user_address)),
            None => Json(ResponseWrapper::message(
                Status::NotFound,
                format!("UserAddress not found with id , {}", id),
            )),
        },
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Updating user_address , {}",
                err.to_string()
            ),
        )),
    }
}
#[delete("/<id>")]
pub fn delete_user_address(
    id: &str,
    db: &State<MongoDBState>,
    middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the user_addresses collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);
    let user_collection: Collection<User> = db.collection::<User>(USERS);

    // Delete the document into MongoDB
    match UserAddress::delete_resource(id, &collection) {
        Ok(_) => {
            let filter = doc! { "_id": ObjectId::from_str(&middleware.0.sub).unwrap() };

            let update = doc! {
                "$pull": { "address": ObjectId::from_str(id).unwrap() }
            };
            match user_collection.find_one_and_update(filter, update, None) {
                Ok(_) => Json(ResponseWrapper::new(
                    Status::Ok,
                    String::from("UserAddress Deleted successfully"),
                )),
                Err(err) => Json(ResponseWrapper::message(
                    Status::NotFound,
                    format!(
                        "Something Went Wrong While Deleting user_address , {}",
                        err.to_string()
                    ),
                )),
            }
        }
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Deleting user_address , {}",
                err.to_string()
            ),
        )),
    }
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::*;

    #[test]
    fn it_should_deserialize_user_address() {
        let mock = json!({
            "firstName":"lana",
            "lastName":"rhodes",
            "country":"Las Vegas",
            "street":{
                "line1":"10th Holly Velley",
                "line2":"Near John Keen Park"
            },
            "state":"Lonize",
            "town":"Milfy",
            "zip":126510,
            "phone":"9985632320",
            "email":"me@mail.co"
        });
        let _: UserAddress = serde_json::from_str(&mock.to_string()).unwrap();
    }
}
