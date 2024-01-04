use std::str::FromStr;

use crate::etc::{hasher::Hasher, json_web_token::JwtService, validations::Validations};
use mongodb::{
    bson::{bson, doc, oid::ObjectId, Bson},
    sync::Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(deserialize_with = "Validations::validate_email")]
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Vec<ObjectId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword<'a> {
    current_password: &'a str,
    password: &'a str,
}

pub type Users = Vec<User>;

impl User {
    pub fn hash(self) -> Self {
        let password = Hasher::hash_password(&self.password.unwrap()).unwrap();
        Self {
            id: None,
            username: self.username,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            address: self.address,
            password: Some(password),
        }
    }
    pub fn login(credentials: &Login, collection: &Collection<User>) -> Result<String, String> {
        let user = collection.find_one(doc! {"email":&credentials.email}, None);
        match user {
            Ok(user) => {
                let user = user.unwrap();
                if Hasher::verify_password(&credentials.password, &user.password.unwrap()) {
                    let token =
                        JwtService::generate_token(&user.id.unwrap().to_string(), 60 * 60 * 24 * 7);
                    return Ok(token);
                }
                Ok(String::from(""))
            }
            Err(_) => Err(String::from("User not found")),
        }
    }

    pub fn change_password<'a>(
        id: &'a str,
        change: ChangePassword<'a>,
        collection: &Collection<User>,
    ) -> Result<Option<User>, ()> {
        let filter = doc! { "_id": ObjectId::from_str(id).unwrap() };

        match collection.find_one(filter.clone(), None) {
            Ok(user) => {
                let user = user.unwrap();
                let is_valid =
                    Hasher::verify_password(&change.current_password, &user.password.unwrap());
                if is_valid {
                    let update = doc! {
                        "$set": { "password": change.password }
                    };
                    return Ok(collection
                        .find_one_and_update(filter.clone(), update, None)
                        .unwrap());
                } else {
                    Err(())
                }
            }
            Err(_) => Err(()),
        }
    }
}

impl Into<Bson> for User {
    fn into(self) -> Bson {
        bson!( {
            "username":self.username,
            "email":self.email,
            "firstName":self.first_name,
            "lastName":self.last_name,
        })
    }
}
