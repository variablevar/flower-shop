use std::{fmt::Debug, str::FromStr};

use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    results::{InsertManyResult, InsertOneResult},
    sync::Collection,
};
use serde::{de::DeserializeOwned, Serialize};

// Get _id
pub fn object_id(id: &str) -> ObjectId {
    ObjectId::from_str(&id).expect("Invalid resource Id")
}
pub trait Generic<
    T: Debug + Clone + PartialEq + Into<Bson> + Serialize + DeserializeOwned + Unpin + Send + Sync,
>
{
    // Insert multiple resource into MongoDB
    fn insert_multiple_resource(
        resources: &Vec<T>,
        collection: &Collection<T>,
    ) -> Result<InsertManyResult, mongodb::error::Error> {
        let resources = collection.insert_many(resources, None)?;
        Ok(resources)
    }

    // Insert resource into MongoDB
    fn insert_resource(
        resource: &T,
        collection: &Collection<T>,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        let resource = collection.insert_one(resource, None)?;
        Ok(resource)
    }

    // Find resource by ID in MongoDBs
    fn find_resource_by_id(
        id: &str,
        collection: &Collection<T>,
    ) -> Result<Option<T>, mongodb::error::Error> {
        let filter = doc! { "_id": object_id(&id) };

        if let Some(document) = collection.find_one(filter, None)? {
            Ok(Some(document))
        } else {
            Ok(None)
        }
    }

    fn find_resource_by_object_id(
        document: ObjectId,
        collection: &Collection<T>,
    ) -> Result<Option<T>, mongodb::error::Error> {
        let filter = doc! { "_id": document };

        if let Some(document) = collection.find_one(filter, None)? {
            Ok(Some(document))
        } else {
            Ok(None)
        }
    }

    fn find_resources(collection: &Collection<T>) -> Result<Option<Vec<T>>, mongodb::error::Error> {
        let filter = doc! {};

        let cursor = collection.find(filter, None)?;
        Ok(Some(
            cursor.map(|resource| resource.unwrap()).collect::<Vec<T>>(),
        ))
    }

    // Update resource in MongoDB
    fn update_resource(
        id: &str,
        updated_resource: &T,
        collection: &Collection<T>,
    ) -> Result<Option<T>, mongodb::error::Error> {
        let filter = doc! { "_id": object_id(&id) };
        let serialized_updated_updated = doc! {"$set":updated_resource};

        if let Some(document) =
            collection.find_one_and_update(filter, serialized_updated_updated, None)?
        {
            Ok(Some(document))
        } else {
            Ok(None)
        }
    }

    // Delete resource from MongoDB
    fn delete_resource(id: &str, collection: &Collection<T>) -> Result<(), mongodb::error::Error> {
        let filter = doc! { "_id": object_id(&id)};

        collection.delete_one(filter, None)?;

        Ok(())
    }
}
