use mongodb::sync::Collection;
use rocket::{serde::json::Json, State};
use serde_json::json;

use crate::{
    constants::strings::BLOGS,
    database::mongo::MongoDBState,
    guards::json_web_token::JwtToken,
    model::blog::{Blog, Blogs},
    services::generic::Generic,
};

#[post("/many", format = "json", data = "<blogs>")]
pub fn create_blogs(blogs: Json<Blogs>, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Insert the document into MongoDB
    match Blog::insert_multiple_resource(&blogs.0, &collection) {
        Ok(_) => {
            json!({ "status": "success", "response": "Blogs were created successfully" })
        }
        Err(_) => json!({ "status": "failed", "message": "Failed to create product" }),
    }
}

#[post("/", format = "json", data = "<blog>")]
pub fn create_blog(blog: Json<Blog>, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Insert the document into MongoDB
    match Blog::insert_resource(&blog.0, &collection) {
        Ok(_) => json!({ "status": "success", "response": "Blog Created successfully" }),
        Err(_) => json!({ "status": "failed", "message": "Failed to create blog" }),
    }
}

#[get("/<id>")]
pub fn get_blog(id: &str, db: &State<MongoDBState>, middleware: JwtToken) -> serde_json::Value {
    // Get the blogs collection
    println!("{}", middleware.0.sub);
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Get the document of blog into MongoDB
    match Blog::find_resource_by_id(id, &collection) {
        Ok(blog) => json!({ "status": "success", "response": blog.unwrap() }),
        Err(_) => json!({ "status": "failed", "message": "Failed to get blog" }),
    }
}

#[get("/")]
pub fn get_blogs(db: &State<MongoDBState>) -> serde_json::Value {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Get the documents of blogs into MongoDB
    match Blog::find_resources(&collection) {
        Ok(blogs) => json!({ "status": "success", "response": blogs.unwrap() }),
        Err(_) => json!({ "status": "failed", "message": "Failed to get blogs" }),
    }
}

#[put("/<id>", format = "json", data = "<blog>")]
pub fn update_blog(id: &str, blog: Json<Blog>, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Update the document into MongoDB
    match Blog::update_resource(id, &blog.0, &collection) {
        Ok(_) => json!({ "status": "success", "response": "Blog updated successfully" }),
        Err(err) => json!({ "status": "failed", "message": err.to_string() }),
    }
}
#[delete("/<id>")]
pub fn delete_blog(id: &str, db: &State<MongoDBState>) -> serde_json::Value {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Delete the document into MongoDB
    match Blog::delete_resource(id, &collection) {
        Ok(_) => json!({ "status": "success", "response": "Blog Deleted successfully" }),
        Err(_) => json!({ "status": "failed", "message": "Failed to delete blog" }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_deserialize_blog() {
        let mock = json!({
            "id": 1,
            "image": "/assets/img/blog/blog-13.jpg",
            "category": ["lifestyle", "men"],
            "title": "A guide to latest trends",
            "url": "/blog-details-standard",
            "author": "Admin",
            "authorUrl": "/blog-standard"
        });
        let _: Blog = serde_json::from_str(&mock.to_string()).unwrap();
    }
}
