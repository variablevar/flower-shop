use mongodb::sync::Collection;
use rocket::{serde::json::Json, State};
use serde_json::json;

use crate::{
    constants::strings::ADDRESSES,
    database::mongo::MongoDBState,
    model::user_address::{UserAddress, UserAddresses},
    services::generic::Generic,
};

#[post("/many", format = "json", data = "<user_addresses>")]
pub fn create_user_addresses(
    user_addresses: Json<UserAddresses>,
    db: &State<MongoDBState>,
) -> serde_json::Value {
    // Get the user_addresss collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);

    // Insert the document into MongoDB
    match UserAddress::insert_multiple_resource(&user_addresses.0, &collection) {
        Ok(_) => {
            json!({ "status": "success", "response": "UserAddresss were created successfully" })
        }
        Err(_) => json!({ "status": "failed", "message": "Failed to create user_address" }),
    }
}

#[post("/", format = "json", data = "<user_address>")]
pub fn create_user_address(
    user_address: Json<UserAddress>,
    db: &State<MongoDBState>,
) -> serde_json::Value {
    // Get the user_addresss collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);

    // Insert the document into MongoDB
    match UserAddress::insert_resource(&user_address.0, &collection) {
        Ok(_) => json!({ "status": "success", "response": "UserAddress Created successfully" }),
        Err(_) => json!({ "status": "failed", "message": "Failed to create user_address" }),
    }
}

#[get("/<id>")]
pub fn get_user_address(id: &str, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the user_addresss collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);

    // Get the document of user_address into MongoDB
    match UserAddress::find_resource_by_id(id, &collection) {
        Ok(user_address) => json!({ "status": "success", "response": user_address.unwrap() }),
        Err(_) => json!({ "status": "failed", "message": "Failed to get user_address" }),
    }
}

#[get("/")]
pub fn get_user_addresses(db: &State<MongoDBState>) -> serde_json::Value {
    // Get the user_addresss collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);

    // Get the documents of user_addresss into MongoDB
    match UserAddress::find_resources(&collection) {
        Ok(user_addresss) => json!({ "status": "success", "response": user_addresss.unwrap() }),
        Err(_) => json!({ "status": "failed", "message": "Failed to get user_addresss" }),
    }
}

#[put("/<id>", format = "json", data = "<user_address>")]
pub fn update_user_address(
    id: &str,
    user_address: Json<UserAddress>,
    db: &State<MongoDBState>,
) -> serde_json::Value {
    // Get the user_addresss collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);

    // Update the document into MongoDB
    match UserAddress::update_resource(id, &user_address.0, &collection) {
        Ok(_) => json!({ "status": "success", "response": "UserAddress updated successfully" }),
        Err(err) => json!({ "status": "failed", "message": err.to_string() }),
    }
}
#[delete("/<id>")]
pub fn delete_user_address(id: &str, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the user_addresss collection
    let db = db.inner().db();
    let collection: Collection<UserAddress> = db.collection::<UserAddress>(ADDRESSES);

    // Delete the document into MongoDB
    match UserAddress::delete_resource(id, &collection) {
        Ok(_) => json!({ "status": "success", "response": "UserAddress Deleted successfully" }),
        Err(_) => json!({ "status": "failed", "message": "Failed to delete user_address" }),
    }
}

#[cfg(test)]
mod tests {

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
