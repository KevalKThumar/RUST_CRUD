use crate::User;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use bson::{doc, oid::ObjectId};
use mongodb::{Client, Collection};

/// Delete a user by ID from the database
pub async fn delete_user(
    Path(id): Path<String>, // Extract `id` from the URL path
    client: Client,
) -> impl IntoResponse {
    let my_coll: Collection<User> = client.database("llm").collection("users");

    match ObjectId::parse_str(&id) {
        Ok(object_id) => match my_coll.delete_one(doc! { "_id": object_id }).await {
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
        },
        Err(_) => (
            StatusCode::BAD_REQUEST,
            (
                StatusCode::OK,
                Json(doc! { "message": format!("Invalid ID format") }),
            )
                .into_response(),
        )
            .into_response(),
    }
}
