use mongodb::{bson::doc, results::InsertOneResult, sync::Collection};

use crate::{
    model::{
        order::{Order, OrderItem},
        product::Product,
    },
    services::generic::object_id,
};

impl Order {
    pub fn place_order(
        order: &Order,
        order_collection: &Collection<Order>,
        product_collection: &Collection<Product>,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        let resource = order_collection.insert_one(order, None).unwrap();
        let products: Vec<Product> = order
            .items
            .iter()
            .map(|order_item: &OrderItem| {
                let product = product_collection
                    .find_one_and_update(
                        doc! { "_id": object_id(order_item.product.as_str()) },
                        doc! {"$inc": { "stock":-order_item.quantity}},
                        None,
                    )
                    .unwrap()
                    .unwrap();
                return product;
            })
            .into_iter()
            .collect();
        Ok(resource)
    }
}
