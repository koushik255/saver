use axum::{extract::Path, Extension};
use std::sync::Arc;
use tokio_rusqlite::Connection;

pub type Db = Arc<Connection>;

pub async fn default(
    Path(param): Path<String>,
    Extension(db): Extension<Db>,
) -> String {
    println!("default route / was touched");
    println!("you sent {}", param);

    let name = param.clone();
    let yob = 2000; 

    let insert_result = db
        .call(move |conn| {
            Ok(conn.execute(
                "INSERT INTO person (name, yob) VALUES (?1, ?2)",
                (&name, &yob),
            )?)
        })
        .await;

    match insert_result {
        Ok(_) => format!("Inserted: {}", param),
        Err(e) => format!("DB error: {}", e),
    }
}

pub async fn db() -> Result<Db, Box<dyn std::error::Error>> {
    let db = Connection::open_in_memory().await?;
    db.call(|conn| {
        Ok(conn.execute(
            "CREATE TABLE person (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                yob  INTEGER,
                data  BLOB
            ) STRICT",
            (),
        )?)
    })
    .await?;
    Ok(Arc::new(db))
}