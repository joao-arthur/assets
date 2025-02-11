use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{services::ServeFile, trace::TraceLayer};

#[tokio::main]
async fn main() {
    tokio::join!(serve());
}

async fn serve() {
    let app = Router::new()
        .route_service("/blog.json", ServeFile::new("./blog.json"))
        .route_service("/portfolio.json", ServeFile::new("./portfolio.json"))
        .route_service("/contents_blog.json", ServeFile::new("./contents_blog.json"))
        .route_service("/contents_portfolio.json", ServeFile::new("./contents_portfolio.json"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3307));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http())).await.unwrap();
}
