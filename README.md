# Rust CRUD Application

This project is a simple CRUD (Create, Read, Update, Delete) application built using Rust, Axum, and MongoDB. The application provides RESTful endpoints to manage user data in a MongoDB database.

## Project Structure

The project is structured as follows:

``` bash
.gitignore
Cargo.toml
src
|-- main.rs
|-- user_info.rs
|-- user_info
| |-- add_user.rs
| |-- delete_user.rs
| |-- get_all_users.rs
| |-- get_user.rs
| |-- update_user.rs
```

## Dependencies

The project uses the following dependencies:

```toml
[dependencies]
mongodb = "3.1.1"
dotenv = "0.15"
futures = "0.3.31"
axum = "0.8.1"
tokio = "1"
chrono = "0.4"
serde = "1"
```

## Running the Application

`Set up MongoDB:` Ensure you have a MongoDB instance running and set the MONGODB_URI and PORT in your .env file.

`Build and Run:` Use Cargo to build and run the application.

```bash
cargo build
cargo run
```

## API Endpoints

### The application provides the following API endpoints

#### Add User

```rust
// Route in [main.rs]
.route("/add-user", {
    let client = client.clone();
    post(move |body| add_user(body, client))
})

// Function in [add_user.rs]
pub async fn add_user(
    Json(user): Json<User>,
    client: Client,
) -> impl IntoResponse {
    // Function implementation
}
```

#### Get User

```rust
// Route in [main.rs]
.route("/get-user/{name}", {
    let client = client.clone();
    get(move |name| get_user(name, client.clone()))
})

// Function in [get_user.rs]
pub async fn get_user(
    Path(name): Path<String>,
    client: Client,
) -> impl IntoResponse {
    // Function implementation
}
```

#### Get All Users

```rust
// Route in [main.rs]
.route("/get-all-users", {
    let client = client.clone();
    get(move || get_all_users(client.clone()))
})

// Function in [get_all_users.rs]
pub async fn get_all_users(client: Client) -> impl IntoResponse {
    // Function implementation
}
```

#### Update User

```rust
// Route in [main.rs]
.route("/update-user/{name}", {
    let client = client.clone();
    patch(move |name, body| update_user(name, body, client))
})

// Function in [update_user.rs]
pub async fn update_user(
    Path(name): Path<String>,
    Json(update_data): Json<User>,
    client: Client,
) -> impl IntoResponse {
    // Function implementation
}
```

#### Delete User

```rust
// Route in [main.rs]
.route("/delete-user/{name}", {
    let client = client.clone();
    delete(move |name| delete_user(name, client))
})

// Function in [delete_user.rs]
pub async fn delete_user(
    Path(name): Path<String>,
    client: Client,
) -> impl IntoResponse {
    // Function implementation
}
```

### Approach

1. Modular Design: The project is divided into modules for each CRUD operation (add_user, get_user, get_all_users, update_user, delete_user). This makes the codebase organized and maintainable.

2. MongoDB Integration: The application uses MongoDB as the database. The MongoDB client is initialized in the main function and passed to each handler function.

3. Error Handling: Each handler function includes error handling to return appropriate HTTP status codes and messages.

4. Environment Variables: The application uses environment variables for configuration (e.g., MONGODB_URI, PORT). These are loaded using the dotenv crate.

5. Asynchronous Programming: The application uses asynchronous programming with the tokio runtime to handle concurrent requests efficiently.

### Conclusion

This project demonstrates a simple yet effective way to build a CRUD application in Rust using Axum and MongoDB. The modular design, error handling, and use of environment variables make it a robust and maintainable solution.

This documentation provides an overview of the project, its structure, dependencies, and detailed explanations of each endpoint with code snippets. It also outlines the approach taken to build the application.
