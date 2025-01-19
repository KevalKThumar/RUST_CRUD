use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use mongodb::{
    bson::doc,
    Client, Collection,
};

use crate::User;

/// Function to get user from the database
pub async fn get_user(
    Path(name): Path<String>, 
    client: Client,
) -> impl IntoResponse {
    let my_coll: Collection<User> = client.database("llm").collection("users");

    match my_coll.find_one(doc! { "name": name }).await {
        Ok(Some(document)) => Json(document).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(doc! { "message": format!("User not found") }),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(doc! { "message": format!("Failed to fetch user: {}", err) }),
        )
            .into_response(),
    }
}
