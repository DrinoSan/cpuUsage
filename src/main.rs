use std::sync::{Arc, Mutex};

use axum::{extract::State, response::{IntoResponse, Html}, routing::get, Json, Router};

use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_get))
        .route("/api/cpus", get(cpus_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new())),
        });

    let server =
        axum::Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(app.into_make_service());

    let addr = server.local_addr();
    println!("Listening on address. {}", addr);

    server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    Html(include_str!("index.html"))
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    use std::fmt::Write;

    let mut s = String::new();

    let mut sys = state.sys.lock().unwrap();
    sys.refresh_all();

    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        let i = i + 1;

        let usage = cpu.cpu_usage();
        writeln!(&mut s, "CPU {i} {usage}% ").unwrap();
    }

    Json(v)
}
