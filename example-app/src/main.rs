use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(hector::router(hector::sources!()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening — GET http://localhost:3000/hector → sources.tar.gz");
    axum::serve(listener, app).await.unwrap();
}
