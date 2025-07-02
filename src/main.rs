use axum::{ routing::get, Extension, Router};


mod blud;

use blud::{default,db,list_people2,find};

#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>>{
    let db = db().await?;
    println!("Hello, world!");
    
    let route = Router::new()
    .route("/add/{*capture}", get(default))
    .route("/list",get(list_people2))
    .route("/find/{*capture}", get(find))
    .layer(Extension(db));

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("server is up an running on {addr}");

    axum::serve(listener,route).await.unwrap();
    Ok(())
}
