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
    username: String,
    #[serde(deserialize_with = "Validations::validate_email")]
    email: String,
    address: Vec<ObjectId>,
    password: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

pub type Users = Vec<User>;

impl User {
    pub fn hash(self) -> Self {
        let password = Hasher::hash_password(&self.password).unwrap();
        Self {
            id: None,
            username: self.username,
            email: self.email,
            address: self.address,
            password: password,
        }
    }
    pub fn login(credentials: &Login, collection: &Collection<User>) -> Result<String, String> {
        let user = collection.find_one(doc! {"email":&credentials.email}, None);
        match user {
            Ok(user) => {
                let user = user.unwrap();
                if Hasher::verify_password(&credentials.password, &user.password) {
                    let token =
                        JwtService::generate_token(&user.id.unwrap().to_string(), 60 * 60 * 24 * 7);
                    return Ok(token);
                }
                Ok(String::from(""))
            }
            Err(_) => Err(String::from("User not found")),
        }
    }
}

impl Into<Bson> for User {
    fn into(self) -> Bson {
        bson!( {
            "username":self.username,
            "email":self.email,
        })
    }
}
