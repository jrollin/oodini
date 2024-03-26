use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};

pub async fn router() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/status/:status", get(status_handler))
}
async fn handler() -> impl IntoResponse {
    "Hello World !"
}

// fallback route
pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing here..")
}

// match some status code to make a proper Response
// nb: we cannot use 'impl IntoResponse' because we have different return Types
async fn status_handler(Path(name): Path<String>) -> Response {
    // get status code from pattern matching
    match name.as_str() {
        // OK
        "200" => (StatusCode::OK, ("Everything is fine".to_string())).into_response(),
        "201" => (StatusCode::CREATED).into_response(),
        "204" => (StatusCode::NO_CONTENT).into_response(),
        // // redirection
        "301" | "308" => Redirect::permanent("http://www.rust-lang.org").into_response(),
        "302" | "307" => Redirect::temporary("/hello").into_response(),
        // // Bad request
        "400" => (StatusCode::BAD_REQUEST, ("Bad request".to_string())).into_response(),
        "401" => (StatusCode::UNAUTHORIZED, ("Unauthorized".to_string())).into_response(),
        "403" => (StatusCode::FORBIDDEN, ("Forbidden".to_string())).into_response(),
        "404" => (StatusCode::NOT_FOUND, ("Not found".to_string())).into_response(),
        "405" => (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method not allowed".to_string(),
        )
            .into_response(),

        "422" => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "Unprocessable entity".to_string(),
        )
            .into_response(),
        // Internal error
        "502" => (StatusCode::BAD_GATEWAY, "Bad gateway".to_string()).into_response(),
        // all others
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response(),
    }
}
