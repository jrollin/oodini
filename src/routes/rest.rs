use axum::{response::IntoResponse, routing::get, Json, Router};

use crate::model::tweet::Tweet;

pub async fn router() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/messages", get(handler_messages))
}

async fn handler() -> Json<&'static str> {
    Json("{message: \"Hello world !\"}")
}

async fn handler_messages() -> impl IntoResponse {
    let messages = vec![
        Tweet {
            id: String::from("idone"),
            body: String::from("Message one"),
        },
        Tweet {
            id: String::from("idtwo"),
            body: String::from("Message two"),
        },
    ];
    Json(serde_json::json!(messages))
}
