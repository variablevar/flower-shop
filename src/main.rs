pub mod constants;
pub mod controllers;
pub mod database;
pub mod etc;
pub mod guards;
pub mod model;
pub mod services;
use constants::strings::{ADDRESSES, BLOGS, ORDERS, PRODUCTS, USERS};
use controllers::{
    blog::{create_blog, create_blogs, delete_blog, get_blog, get_blogs, update_blog},
    order::place_order,
    product::{
        create_product, create_products, delete_product, get_product, get_products, update_product,
    },
    user::{change_password, delete_user, get_user, login, register, update_user},
    user_address::{
        create_user_address, create_user_addresses, delete_user_address, get_user_address,
        get_user_addresses, update_user_address,
    },
};
use database::mongo::MongoDBState;
use etc::{
    catchers::{
        bad_request_error, conflict_error, internal_server_error, not_found_error,
        unauthorized_error, unprocessable_entity_error,
    },
    cors::CORS,
};

extern crate serde;
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    rocket::build()
        .attach(CORS::init())
        .register(
            "/",
            rocket::catchers![
                unauthorized_error,
                bad_request_error,
                not_found_error,
                unprocessable_entity_error,
                conflict_error,
                internal_server_error
            ],
        )
        .manage(MongoDBState::init())
        .mount(
            format!("/{}", PRODUCTS),
            routes![
                create_product,
                create_products,
                get_product,
                get_products,
                update_product,
                delete_product
            ],
        )
        .mount(format!("/{}", ORDERS), routes![place_order])
        .mount(
            format!("/{}", BLOGS),
            routes![
                create_blog,
                create_blogs,
                get_blog,
                get_blogs,
                update_blog,
                delete_blog
            ],
        )
        .mount(
            format!("/{}", USERS),
            routes![
                register,
                login,
                get_user,
                update_user,
                delete_user,
                change_password
            ],
        )
        .mount(
            format!("/{}", ADDRESSES),
            routes![
                create_user_address,
                create_user_addresses,
                get_user_address,
                get_user_addresses,
                update_user_address,
                delete_user_address
            ],
        )
        .launch()
        .await
        .unwrap();
}
