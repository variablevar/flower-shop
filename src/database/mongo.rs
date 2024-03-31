use mongodb::{
    bson::doc,
    options::IndexOptions,
    sync::{Client, Collection, Database},
    IndexModel,
};

use crate::{constants::strings::USERS, model::user::User};

pub struct MongoDBState {
    db: Database,
}

impl MongoDBState {
    pub fn init() -> Self {
        let uri = "mongodb+srv://dnfqqjv:PifjSQexkoTGTqPZ@flower-shop.66vvkxh.mongodb.net/?retryWrites=true&w=majority";
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("flower-shop");
        let collection: Collection<User> = db.collection::<User>(USERS);

        let index_options = IndexOptions::builder().unique(true).build();
        let email_index = IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(index_options.clone())
            .build();
        let username_index = IndexModel::builder()
            .keys(doc! { "username": 1 })
            .options(index_options.clone())
            .build();

        collection.create_index(email_index, None).unwrap();
        collection.create_index(username_index, None).unwrap();

        Self { db }
    }

    pub fn db(&self) -> &Database {
        &self.db
    }
}
