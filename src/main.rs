use axum::extract::Query;
use axum::http::StatusCode;
use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::tracing::info;
use lambda_http::{run, tracing, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env::set_var;

#[derive(Deserialize, Serialize)]
struct Params {
    first: Option<String>,
    second: Option<String>,
}

async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /"}))
}

async fn get_foo() -> Json<Value> {
    Json(json!({ "msg": "I am GET /foo" }))
}

async fn post_foo() -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo"}))
}

async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("I am POST /foo/:name, name={name}")}))
}

async fn get_parameters(Query(params): Query<Params>) -> Json<Value> {
    Json(json!({ "request parameters": params }))
}

/// Example on how to return status codes and data from an Axum function
async fn health_check() -> (StatusCode, String) {
    let health = true;
    match health {
        true => (StatusCode::OK, "Healthy!".to_string()),
        false => (StatusCode::INTERNAL_SERVER_ERROR, "Not healthy!".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // API Gatewayで /dev/ や /prod/ とうのステージ名をパスに含めておきたい場合に true にする
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "false");

    // CloudWatch へのログ出力を許可
    tracing::init_default_subscriber();

    info!("Lambda function executed");

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/:name", post(post_foo_name))
        .route("/parameters", get(get_parameters))
        .route("/health/", get(health_check));

    run(app).await
}