use mongodb::sync::Collection;
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    constants::strings::BLOGS,
    database::mongo::MongoDBState,
    guards::jwt_token::JwtToken,
    model::{
        blog::{Blog, Blogs},
        wrapper::ResponseWrapper,
    },
    services::generic::Generic,
};

#[post("/many", format = "json", data = "<blogs>")]
pub fn create_blogs(
    blogs: Json<Blogs>,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Insert the document into MongoDB
    match Blog::insert_multiple_resource(&blogs.0, &collection) {
        Ok(_) => Json(ResponseWrapper::new(
            Status::Ok,
            String::from("Blogs were created successfully"),
        )),
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Creating blog , {}",
                err.to_string()
            ),
        )),
    }
}

#[post("/", format = "json", data = "<blog>")]
pub fn create_blog(
    blog: Json<Blog>,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Insert the document into MongoDB
    match Blog::insert_resource(&blog.0, &collection) {
        Ok(_) => Json(ResponseWrapper::new(
            Status::Ok,
            String::from("Blog Created successfully"),
        )),
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Creating blog , {}",
                err.to_string()
            ),
        )),
    }
}

#[get("/<id>")]
pub fn get_blog<'a>(
    id: &str,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<Blog>> {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Get the document of blog into MongoDB
    match Blog::find_resource_by_id(id, &collection) {
        Ok(db_blog) => match db_blog {
            Some(blog) => Json(ResponseWrapper::new(Status::Ok, blog)),
            None => Json(ResponseWrapper::message(
                Status::NotFound,
                format!("Blog not found with id , {}", id),
            )),
        },
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Getting blog , {}",
                err.to_string()
            ),
        )),
    }
}

#[get("/")]
pub fn get_blogs(db: &State<MongoDBState>, _middleware: JwtToken) -> Json<ResponseWrapper<Blogs>> {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Get the documents of blogs into MongoDB
    match Blog::find_resources(&collection) {
        Ok(db_blogs) => match db_blogs {
            Some(blogs) => Json(ResponseWrapper::new(Status::Accepted, blogs)),
            None => Json(ResponseWrapper::message(
                Status::NotFound,
                format!("Blogs Not Found"),
            )),
        },
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Getting blog , {}",
                err.to_string()
            ),
        )),
    }
}

#[put("/<id>", format = "json", data = "<blog>")]
pub fn update_blog(
    id: &str,
    blog: Json<Blog>,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<Blog>> {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Update the document into MongoDB
    match Blog::update_resource(id, &blog.0, &collection) {
        Ok(db_blog) => match db_blog {
            Some(blog) => Json(ResponseWrapper::new(Status::Ok, blog)),
            None => Json(ResponseWrapper::message(
                Status::NotFound,
                format!("Blog not found with id , {}", id),
            )),
        },
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Updating blog , {}",
                err.to_string()
            ),
        )),
    }
}
#[delete("/<id>")]
pub fn delete_blog(
    id: &str,
    db: &State<MongoDBState>,
    _middleware: JwtToken,
) -> Json<ResponseWrapper<String>> {
    // Get the blogs collection
    let db = db.inner().db();
    let collection: Collection<Blog> = db.collection::<Blog>(BLOGS);

    // Delete the document into MongoDB
    match Blog::delete_resource(id, &collection) {
        Ok(_) => Json(ResponseWrapper::message(
            Status::Ok,
            format!("Blog deleted successfully"),
        )),
        Err(err) => Json(ResponseWrapper::message(
            Status::NotFound,
            format!(
                "Something Went Wrong While Getting blog , {}",
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
