use crate::User;
use axum::{http::StatusCode, response::IntoResponse, Json};
use bson::doc;
use futures::TryStreamExt;
use mongodb::{Client, Collection};
/// Function to get all users from the database
pub async fn get_all_users(client: Client) -> impl IntoResponse {
    let my_coll: Collection<User> = client.database("llm").collection("users");

    match my_coll.find(doc! {}).await {
        Ok(cursor) => {
            let users: Vec<User> = cursor.try_collect().await.unwrap_or_default();
            Json(users).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(doc! { "message": format!("Failed to fetch users: {}", err) }),
        )
            .into_response(),
    }
}