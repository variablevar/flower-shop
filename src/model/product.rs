use std::collections::HashMap;

use mongodb::bson::{bson, doc, oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

use super::category::Category;

pub type Products = Vec<Product>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    stock_keeper_unit: String,
    name: String,
    price: f64,
    discount: i64,
    new: bool,
    rating: i64,
    sale_count: i64,
    category: Vec<Category>,
    tag: Vec<Category>,
    stock: i64,
    image: Vec<String>,
    short_description: String,
    full_description: String,
    other: HashMap<String, String>,
}

impl Into<Bson> for Product {
    fn into(self) -> Bson {
        bson!( {
            "stockKeeperUnit": self.stock_keeper_unit,
            "name": self.name,
            "price": self.price,
            "discount": self.discount,
            "new": self.new,
            "rating": self.rating,
            "saleCount": self.sale_count,
            "category": self.category,
            "tag": self.tag,
            "stock": self.stock,
            "image": self.image,
            "shortDescription": self.short_description,
            "fullDescription": self.full_description,
            "other":  Bson::Document(self.other.into_iter().map(|(key, value)| (key, Bson::String(value))).collect())
        })
    }
}
