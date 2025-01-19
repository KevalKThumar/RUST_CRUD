use crate::User;
use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::{bson::doc, Client, Collection};

/// Function to add a user and return the result as an HTTP response
pub async fn add_user(
    Json(user): Json<User>, // Extract user data from the request body
    client: Client,
) -> impl IntoResponse {
    let my_coll: Collection<User> = client.database("llm").collection("users");

    // check if the user already exists
    if let Ok(document) = my_coll
        .find_one(doc! { "name" : user.name.clone(), "email": user.email.clone() ,})
        .await
    {
        if document.is_some() {
            return (
                StatusCode::CONFLICT,
                Json(doc! { "message": "User already exists" }),
            )
                .into_response();
        }
    }

    match my_coll.insert_one(user).await {
        Ok(res) => {
            // Return inserted ID as a success response
            let response = doc! {
                "inserted_id": res.inserted_id,
                "message": "User inserted successfully"
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(err) => {
            // Handle errors gracefully
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(doc! { "message": format!("Failed to insert user: {}", err) }),
            )
                .into_response()
        }
    }
}
