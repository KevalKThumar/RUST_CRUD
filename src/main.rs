use axum::{
    routing::{get, post, put, delete},
    Router,
};
use dotenv::dotenv;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
// Import the user_info module
mod user_info;
use user_info::add_user::add_user;
use user_info::delete_user::delete_user;
use user_info::get_all_users::get_all_users;
use user_info::get_user::get_user;
use user_info::update_user::update_user;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    role: Option<String>,
    number: Option<String>,
}

/// Main function to run the server and define the routes
#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();
    let mongo_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let client = Client::with_uri_str(mongo_uri).await?;

    let app = Router::new()
        .route("/", get(root)) // Root endpoint
        .route("/get-user/{id}", {
            let client = client.clone();
            get(move |id| get_user(id, client.clone()))
        })
        .route("/get-all-users", {
            let client = client.clone();
            get(move || get_all_users(client.clone()))
        })
        .route("/add-user", {
            let client = client.clone();
            post(move |body| add_user(body, client))
        })
        .route("/update-user/{id}", {
            let client = client.clone();
            put(move |id, body| update_user(id, body, client))
        })
        .route("/delete-user/{id}", {
            let client = client.clone();
            delete(move |id| delete_user(id, client))
        }); // Update user endpoint

    // Run the server
    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().expect("Invalid port")));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

/// Root endpoint to return a simple message
async fn root() -> &'static str {
    "Hello, World!"
}