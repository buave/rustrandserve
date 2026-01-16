use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use base64;
use base64::{engine::general_purpose, Engine as _};
use std::{env, path::PathBuf};
use tokio::fs;

async fn handle_request(Path(filename): Path<String>) -> impl IntoResponse {
    let file_path = PathBuf::from(format!("data/{}", filename));

    if file_path.exists() {
        match fs::read(&file_path).await {
            Ok(content) => Response::builder()
                .status(StatusCode::OK)
                .body(axum::body::Body::from(content))
                .unwrap(),
            Err(_) => Response::builder()
                .status(StatusCode::OK)
                .body(axum::body::Body::from("Error reading file"))
                .unwrap(),
        }
    } else {
        Response::builder()
            .status(StatusCode::OK)
            .body(axum::body::Body::from(generate_random_string(filename)))
            .unwrap()
    }
}

async fn index() -> impl IntoResponse {
    generate_random_string("index".to_string())
}

fn generate_random_string(value: String) -> String {
    let size = ((value.len() + 67) * 3) / 2;
    let mut base64 = general_purpose::STANDARD.encode(value);
    for _ in 0..14 {
        base64 = general_purpose::STANDARD.encode(base64);
    }

    let cut_base64 = &base64[..size];
    let mut chars: Vec<char> = cut_base64.chars().collect();
    let len_base64 = cut_base64.len();

    for r in 0..(size / 2) {
        for i in 0..len_base64 {
            let j = (i + r + 3) % len_base64;
            chars.swap(i, j);
        }
    }

    format!("{}\n", String::from_iter(chars))
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut port = "8989";

    if args.len() > 1 {
        port = &args[1];
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/{*path}", get(handle_request));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
