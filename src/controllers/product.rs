use mongodb::sync::Collection;
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    constants::strings::PRODUCTS,
    database::mongo::MongoDBState,
    guards::jwt_token::JwtToken,
    model::{
        product::{Product, Products},
        wrapper::ResponseWrapper,
    },
    services::generic::Generic,
};

#[post("/many", format = "json", data = "<products>")]
pub fn create_products(
    products: Json<Products>,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Insert the document into MongoDB
    match Product::insert_multiple_resource(&products.0, &collection) {
        Ok(_) => Json(ResponseWrapper::new(
            Status::Ok,
            String::from("Products were created successfully"),
        )),
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Creating product , {}",
                err.to_string()
            ),
        )),
    }
}

#[post("/", format = "json", data = "<product>")]
pub fn create_product(
    product: Json<Product>,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Insert the document into MongoDB
    match Product::insert_resource(&product.0, &collection) {
        Ok(_) => Json(ResponseWrapper::new(
            Status::Ok,
            String::from("Product Created successfully"),
        )),
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Creating product , {}",
                err.to_string()
            ),
        )),
    }
}

#[get("/<id>")]
pub fn get_product<'a>(
    id: &str,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<Product>> {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Get the document of product into MongoDB
    match Product::find_resource_by_id(id, &collection) {
        Ok(db_product) => match db_product {
            Some(product) => Json(ResponseWrapper::new(Status::Ok, product)),
            None => Json(ResponseWrapper::message(
                Status::NotFound,
                format!("Product not found with id , {}", id),
            )),
        },
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Getting product , {}",
                err.to_string()
            ),
        )),
    }
}

#[get("/")]
pub fn get_products(
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<Products>> {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Get the documents of products into MongoDB
    match Product::find_resources(&collection) {
        Ok(db_products) => match db_products {
            Some(products) => Json(ResponseWrapper::new(Status::Accepted, products)),
            None => Json(ResponseWrapper::message(
                Status::NotFound,
                format!("Products Not Found"),
            )),
        },
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Getting product , {}",
                err.to_string()
            ),
        )),
    }
}

#[put("/<id>", format = "json", data = "<product>")]
pub fn update_product(
    id: &str,
    product: Json<Product>,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<Product>> {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Update the document into MongoDB
    match Product::update_resource(id, &product.0, &collection) {
        Ok(db_product) => match db_product {
            Some(product) => Json(ResponseWrapper::new(Status::Ok, product)),
            None => Json(ResponseWrapper::message(
                Status::NotFound,
                format!("Product not found with id , {}", id),
            )),
        },
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Updating product , {}",
                err.to_string()
            ),
        )),
    }
}
#[delete("/<id>")]
pub fn delete_product(
    id: &str,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the products collection
    let db = db.inner().db();
    let collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Delete the document into MongoDB
    match Product::delete_resource(id, &collection) {
        Ok(_) => Json(ResponseWrapper::message(
            Status::Ok,
            format!("Product deleted successfully"),
        )),
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Getting product , {}",
                err.to_string()
            ),
        )),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

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
