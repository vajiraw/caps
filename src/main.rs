use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct Greeting {
    message: String,
}

#[derive(Deserialize)]
struct GreetingRequest {
    name: String,
}

async fn get_greeting() -> Json<Greeting> {
    let response = Greeting {
        message: "Hello, world!".to_string(),
    };
    Json(response)
}

async fn personalized_greeting(Json(payload): Json<GreetingRequest>) -> Json<Greeting> {
    let response = Greeting {
        message: format!("Hello, {}!", payload.name),
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    // Define the routes
    let app = Router::new()
        .route("/", get(get_greeting))
        .route("/greet", post(personalized_greeting));

    // Specify the address and port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
