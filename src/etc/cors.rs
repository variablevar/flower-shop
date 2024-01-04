use rocket::http::Method;
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};
pub struct CORS;

impl CORS {
    // Function to handle CORS preflight requests
    pub fn init() -> Cors {
        let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

        CorsOptions {
            allowed_origins,
            allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()
        .expect("Error creating CORS configuration")
    }
}
