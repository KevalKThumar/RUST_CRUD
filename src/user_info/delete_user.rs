use crate::User;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use mongodb::{
    bson::doc,
    Client, Collection,
};

/// Delete a user by ID from the database
pub async fn delete_user(
    Path(name): Path<String>, // Extract `id` from the URL path
    client: Client,
) -> impl IntoResponse {
    let my_coll: Collection<User> = client.database("llm").collection("users");

    match my_coll.delete_one(doc! { "name": name }).await {
        Ok(delete_result) => {
            if delete_result.deleted_count > 0 {
                (
                    StatusCode::OK,
                    Json(doc! { "message": format!("User deleted successfully") }),
                )
                    .into_response()
            } else {
                (
                    StatusCode::NOT_FOUND,
                    Json(doc! { "message": format!("User not found") }),
                )
                    .into_response()
            }
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(doc! { "message": format!("Failed to delete user: {}", err), }),
        )
            .into_response(),
    }
}
