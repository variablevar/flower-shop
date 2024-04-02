use mongodb::sync::{Client, Database};

pub struct MongoDBState {
    db: Database,
}

impl MongoDBState {
    pub fn init() -> Self {
        let uri = "mongodb+srv://dnfqqjv:PifjSQexkoTGTqPZ@flower-shop.66vvkxh.mongodb.net/?retryWrites=true&w=majority";
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("flower-shop");

        Self { db }
    }

    pub fn db(&self) -> &Database {
        &self.db
    }
}
