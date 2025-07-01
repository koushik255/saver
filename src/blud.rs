use axum::{extract::Path, Extension};
use std::sync::Arc;
use tokio_rusqlite::Connection;

#[derive(Debug)]
struct Save {
    id: i64,
    name: String,
    yob: i64,
}

pub type Db = Arc<Connection>;

pub async fn default(
    Path(param): Path<String>,
    // receives/cloning the arc
    Extension(db): Extension<Db>,
) -> String {
    
    let name = param.clone();
    let yob = 2000;
    println!("inserted {}", name);
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
    let db = Connection::open("my_db").await?;
    db.call(|conn| {
        Ok(conn.execute(
            "CREATE TABLE IF NOT EXISTS person (
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

pub async fn list_people(Extension(db): Extension<Db>) -> String {
    println!("listing people");
    let people = db
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT id, name, yob FROM person")?;
            let rows = stmt
                .query_map([], |row| {
                    Ok(Save {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        yob: row.get(2)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
        .await;

    match people {
        Ok(people) => {
            if people.is_empty() {
                "No people found.".to_string()
            } else {
                people
                    .into_iter()
                    .map(|p| format!("{}: {} ({})", p.id, p.name, p.yob))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
        Err(e) => format!("DB error: {}", e),
    }
}