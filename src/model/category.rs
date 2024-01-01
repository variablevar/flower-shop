use mongodb::bson::{bson, Bson};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Flower,
    Plant,
}
impl Into<Bson> for Category {
    fn into(self) -> Bson {
        match self {
            Category::Flower => bson!("flower"),
            Category::Plant => bson!("plant"),
        }
    }
}
