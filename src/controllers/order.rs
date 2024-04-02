use mongodb::sync::Collection;
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    constants::strings::{ORDERS, PRODUCTS},
    database::mongo::MongoDBState,
    guards::jwt_token::JwtToken,
    model::{order::Order, product::Product, wrapper::ResponseWrapper},
};

#[post("/", format = "json", data = "<order>")]
pub fn place_order(
    mut order: Json<Order>,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the products collection
    let db = db.inner().db();
    let order_collection: Collection<Order> = db.collection::<Order>(ORDERS);
    let product_collection: Collection<Product> = db.collection::<Product>(PRODUCTS);

    // Insert the document into MongoDB
    let _ = &order.set_user(_middleware.0.sub);
    match Order::place_order(&order.0, &order_collection, &product_collection) {
        Ok(_) => Json(ResponseWrapper::new(
            Status::Ok,
            String::from("Order Placed successfully"),
        )),
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Placeing Order , {}",
                err.to_string()
            ),
        )),
    }
}
