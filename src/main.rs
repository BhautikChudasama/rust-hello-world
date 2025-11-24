use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new().route("/", get(handler));

    // Run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

