use crate::etc::validations::Validations;
use mongodb::bson::{bson, oid::ObjectId, Bson};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAddress {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    first_name: String,
    last_name: String,
    country: String,
    street: Street,
    state: String,
    town: String,
    #[serde(deserialize_with = "Validations::validate_zip_code")]
    zip: i64,
    #[serde(deserialize_with = "Validations::validate_mobile_number")]
    phone: String,
    #[serde(deserialize_with = "Validations::validate_email")]
    email: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Street {
    line1: String,
    line2: String,
}
pub type UserAddresses = Vec<UserAddress>;

impl Into<Bson> for UserAddress {
    fn into(self) -> Bson {
        bson!( {
            "firstName":self.first_name,
            "lastName":self.last_name,
            "country":self.country,
            "street":{
                "line1":self.street.line1,
                "line2":self.street.line2
            },
            "state":self.state,
            "town":self.town,
            "zip":self.zip,
            "phone":self.phone,
            "email":self.email
        })
    }
}
