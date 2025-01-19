use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use mongodb::{bson::doc, Client, Collection};

use crate::User;
/// Function to update user in the database by ID
#[warn(unused_variables)]
pub async fn update_user(
    Path(name): Path<String>,      // Extract `id` from the URL path
    Json(update_data): Json<User>, // Extract partial update data
    client: Client,                // MongoDB client
) -> impl IntoResponse {
    let collection: Collection<User> = client.database("llm").collection("users");

    // Create a filter to locate the document to update
    let filter = doc! { "name": name };

    // Dynamically build the `$set` update document
    let mut update_doc = doc! {};
    if let Some(name) = update_data.name {
        // You can not update the name of a user
        return (
            StatusCode::BAD_REQUEST,
            Json(doc! { "message": format!("You can not update the name of a user") }),
        )
            .into_response();
    }
    if let Some(email) = update_data.email {
        return (
            StatusCode::BAD_REQUEST,
            Json(doc! { "message": format!("You can not update the email of a user") }),
        )
            .into_response();
    }
    if let Some(password) = update_data.password {
        update_doc.insert("password", password);
    }
    if let Some(role) = update_data.role {
        update_doc.insert("role", role);
    }
    if let Some(number) = update_data.number {
        update_doc.insert("number", number);
    }

    // Ensure there is something to update
    if update_doc.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(doc! { "message": format!("No update data provided") }),
        )
            .into_response();
    }

    let update = doc! { "$set": update_doc };

    // Perform the update
    match collection.update_one(filter, update).await {
        Ok(update_result) => {
            if update_result.matched_count == 0 {
                (
                    StatusCode::NOT_FOUND,
                    Json(doc! { "message": format!("User not found") }),
                )
                    .into_response()
            } else {
                (
                    StatusCode::OK,
                    Json(doc! { "message": format!("User updated successfully") }),
                )
                    .into_response()
            }
        }
        Err(err) => {
            eprintln!("Database error: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update user").into_response()
        }
    }
}
