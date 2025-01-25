use axum::{routing::get, Router, response::IntoResponse, extract::Path};
use rand::{distributions::Alphanumeric, Rng};
use tokio::fs;
use std::path::PathBuf;

async fn handle_request(Path(filename): Path<String>) -> impl IntoResponse {
    let file_path = PathBuf::from(format!("data/{}", filename));

    if file_path.exists() {
        match fs::read_to_string(&file_path).await {
            Ok(content) => return content,
            Err(_) => return "Error reading file".to_string(),
        }
    }

    generate_random_string()
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
    let app = Router::new()
        .route("/", get(index))
        .route("/*path", get(handle_request));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
