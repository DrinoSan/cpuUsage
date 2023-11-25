use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello World!" }));

    let server =
        axum::Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(app.into_make_service());

    let addr = server.local_addr();
    println!("Listening on address. {}", addr);

    server.await.unwrap();
}
