use axum::{ routing::get, Extension, Router};
use tower_http::cors::{CorsLayer,Any};


mod blud;

use blud::{db, default, find, find_name, list_people, list_people2};

#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>>{
    let db = db().await?;
    println!("Hello, world!");
    

    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

    let route = Router::new()
    .route("/add/{*capture}", get(default))
    .route("/list",get(list_people2))
    .route("/blud", get(list_people))
    .route("/find/{*capture}", get(find))
    .route("/search/{*capture}", get(find_name))
    .layer(Extension(db))
    .layer(cors);

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("server is up an running on {addr}");

    axum::serve(listener,route).await.unwrap();
    Ok(())
}
