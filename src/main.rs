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
        .route_service("/contents_portfolio.json", ServeFile::new("./contents_portfolio.json"))
        .route_service("/blog/html-and-components/en.md", ServeFile::new("./blog/html-and-components/en.md"))
        .route_service("/blog/html-and-components/pt.md", ServeFile::new("./blog/html-and-components/pt.md"))
        .route_service("/blog/interfaces/en.md", ServeFile::new("./blog/interfaces/en.md"))
        .route_service("/blog/interfaces/pt.md", ServeFile::new("./blog/interfaces/pt.md"))
        .route_service("/blog/layered-architecture-ddd-and-market/en.md", ServeFile::new("./blog/layered-architecture-ddd-and-market/en.md"))
        .route_service("/blog/layered-architecture-ddd-and-market/pt.md", ServeFile::new("./blog/layered-architecture-ddd-and-market/pt.md"))
        .route_service("/blog/unix-like-and-packages/en.md", ServeFile::new("./blog/unix-like-and-packages/en.md"))
        .route_service("/blog/unix-like-and-packages/pt.md", ServeFile::new("./blog/unix-like-and-packages/pt.md"))
        .route_service("/portfolio/libre_game_of_life/en.md", ServeFile::new("./portfolio/libre_game_of_life/en.md"))
        .route_service("/portfolio/libre_game_of_life/pt.md", ServeFile::new("./portfolio/libre_game_of_life/pt.md"))
        .route_service("/portfolio/precise_schedule/en.md", ServeFile::new("./portfolio/precise_schedule/en.md"))
        .route_service("/portfolio/precise_schedule/pt.md", ServeFile::new("./portfolio/precise_schedule/pt.md"));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3307));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http())).await.unwrap();
}
