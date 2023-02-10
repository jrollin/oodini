use dotenv::dotenv;
use std::net::SocketAddr;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // loads the environment variables from the ".env" file.
    dotenv().ok();
    // get listening port
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    // ensure port is valid type
    let port: u16 = port.parse().expect("Port should be valid range");
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/hello/:name", get(hello_handler))
        .route("/status/:status", get(status_handler));
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("Hello World !")
}

async fn hello_handler(Path(name): Path<String>) -> impl IntoResponse {
    let greeting = name.as_str();
    let hello = String::from("Hello ");

    (StatusCode::OK, Html(hello + greeting))
}

async fn status_handler(Path(name): Path<String>) -> impl IntoResponse {
    match name.as_str() {
        "400" => (StatusCode::NOT_FOUND, Html("Not found".to_string())),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("Internal server error".to_string()),
        ),
    }
}
