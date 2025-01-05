use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use bson::{doc, oid::ObjectId};
use mongodb::{Client, Collection};

use crate::User;

/// Function to get user from the database
pub async fn get_user(
    Path(id): Path<String>, // Extract `id` from the URL path
    client: Client,
) -> impl IntoResponse {
    let my_coll: Collection<User> = client.database("llm").collection("users");

    match ObjectId::parse_str(&id) {
        Ok(object_id) => match my_coll.find_one(doc! { "_id": object_id }).await {
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
        },
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(doc! { "message": "Invalid ID format" }),
        )
            .into_response(),
    }
}
