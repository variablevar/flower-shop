use mongodb::sync::Collection;
use rocket::{serde::json::Json, State};
use serde_json::json;

use crate::{
    constants::strings::PRODUCTS,
    database::mongo::MongoDBState,
    model::product::{Product, Products},
    services::generic::Generic,
};

#[post("/many", format = "json", data = "<products>")]
pub fn create_products(products: Json<Products>, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Insert the document into MongoDB
    match Product::insert_multiple_resource(&products.0, &collection) {
        Ok(_) => {
            json!({ "status": "success", "response": "Products were created successfully" })
        }
        Err(_) => json!({ "status": "failed", "message": "Failed to create product" }),
    }
}

#[post("/", format = "json", data = "<product>")]
pub fn create_product(product: Json<Product>, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Insert the document into MongoDB
    match Product::insert_resource(&product.0, &collection) {
        Ok(_) => json!({ "status": "success", "response": "Product Created successfully" }),
        Err(_) => json!({ "status": "failed", "message": "Failed to create product" }),
    }
}

#[get("/<id>")]
pub fn get_product(id: &str, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Get the document of product into MongoDB
    match Product::find_resource_by_id(id, &collection) {
        Ok(product) => json!({ "status": "success", "response": product.unwrap() }),
        Err(_) => json!({ "status": "failed", "message": "Failed to get product" }),
    }
}

#[get("/")]
pub fn get_products(db: &State<MongoDBState>) -> serde_json::Value {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Get the documents of products into MongoDB
    match Product::find_resources(&collection) {
        Ok(products) => json!({ "status": "success", "response": products.unwrap() }),
        Err(_) => json!({ "status": "failed", "message": "Failed to get products" }),
    }
}

#[put("/<id>", format = "json", data = "<product>")]
pub fn update_product(
    id: &str,
    product: Json<Product>,
    db: &State<MongoDBState>,
) -> serde_json::Value {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Update the document into MongoDB
    match Product::update_resource(id, &product.0, &collection) {
        Ok(_) => json!({ "status": "success", "response": "Product updated successfully" }),
        Err(err) => json!({ "status": "failed", "message": err.to_string() }),
    }
}
#[delete("/<id>")]
pub fn delete_product(id: &str, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Delete the document into MongoDB
    match Product::delete_resource(id, &collection) {
        Ok(_) => json!({ "status": "success", "response": "Product Deleted successfully" }),
        Err(_) => json!({ "status": "failed", "message": "Failed to delete product" }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_deserialize_product() {
        let mock = json!({
          "stockKeeperUnit": "asdf148",
          "name": "Lorem ipsum plant one",
          "price": 15.6,
          "discount": 0,
          "new": false,
          "rating": 4,
          "saleCount": 90,
          "category": ["plant"],
          "tag": ["plant"],
          "stock": 15,
          "image": [
            "/assets/img/product/plants/1.jpg",
            "/assets/img/product/plants/2.jpg",
            "/assets/img/product/plants/3.jpg",
            "/assets/img/product/plants/4.jpg",
            "/assets/img/product/plants/5.jpg"
          ],
          "shortDescription": "Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur.",
          "fullDescription": "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt."
        });
        let _: Product = serde_json::from_str(&mock.to_string()).unwrap();
    }
}
