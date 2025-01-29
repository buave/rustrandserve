use axum::{
    routing::get, 
    Router, 
    response::{IntoResponse, Response}, 
    extract::Path,
    http::{StatusCode, header::CONTENT_TYPE},
};
use rand::{distributions::Alphanumeric, Rng};
use tokio::fs;
use std::{env, path::PathBuf};
use mime_guess;

async fn handle_request(Path(filename): Path<String>) -> impl IntoResponse {
    let file_path = PathBuf::from(format!("data/{}", filename));

    if file_path.exists() {
        match fs::read(&file_path).await {
            Ok(content) => {
                let mime_type = mime_guess::from_path(&file_path).first_or_octet_stream();
                Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, mime_type.as_ref())
                    .body(axum::body::Body::from(content))
                    .unwrap()
            }
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("Error reading file"))
                .unwrap(),
        }
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::Body::from(generate_random_string()))
            .unwrap()
    }
}

async fn index() -> impl IntoResponse {
    generate_random_string()
}

fn generate_random_string() -> String {
    let size = rand::thread_rng().gen_range(50..1000);
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    let mut port = "9999";

    if args.len() > 1 {
        port = &args[1];
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/{*path}", get(handle_request));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}