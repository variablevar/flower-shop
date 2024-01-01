use mongodb::bson::{bson, oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

pub type Blogs = Vec<Blog>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    image: String,
    category: Vec<String>,
    title: String,
    url: String,
    author: String,
    author_url: String,
}

impl Into<Bson> for Blog {
    fn into(self) -> Bson {
        bson!({
            "image": self.image ,
            "category": self.category ,
            "title": self.title ,
            "url": self.url ,
            "author": self.author ,
            "authorUrl": self.author_url
        })
    }
}
