use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};

use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let app_state = AppState::default();

    let app = Router::new()
        .route("/", get(root_get))
        .route("/index.mjs", get(indexmjs_get))
        .route("/index.css", get(indexcss_get))
        .route("/api/cpus", get(cpus_get))
        .with_state(app_state.clone());

    // Update CPU usage state in background
    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_all();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

            {
                let mut cpus = app_state.cpus.lock().unwrap();
                *cpus = v;
            }

            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    let server =
        axum::Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(app.into_make_service());

    let addr = server.local_addr();
    println!("Listening on address. {}", addr);

    server.await.unwrap();
}

#[derive(Clone, Default)]
struct AppState {
    cpus: Arc<Mutex<Vec<f32>>>,
}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html")
        .await
        .expect("Cant find index.html");

    Html(markup)
}

#[axum::debug_handler]
async fn indexmjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.mjs")
        .await
        .expect("Cant find index.mjs");

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn indexcss_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.css")
        .await
        .expect("Cant find index.css");

    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let v = state.cpus.lock().unwrap().clone();
    Json(v)
}
