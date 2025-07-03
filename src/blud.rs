use axum::{debug_handler, extract::Path,Json, Extension,};
use std::sync::Arc;
use tokio_rusqlite::Connection;
use serde::Serialize;

#[derive(Debug,Serialize,Clone)]
pub struct Save {
    pub id: i64,
    pub name: String,
    pub post: String,
    pub yob: i64,
}

pub type Db = Arc<Connection>;


pub async fn getnd(Path(param): Path<String>, Extension(_db): Extension<Db>,) -> String {
    let string_to_split = param.clone();

    // splitting at | then setting it as a iterator then unraping
    let ndpart = string_to_split.split("|").nth(1).unwrap_or("");
    println!("2nd part {:?}", ndpart);

    ndpart.to_string()
}



pub async fn getfirst(Path(param): Path<String>, Extension(_db): Extension<Db>,) -> String {

    let newparam = param.split_once('|');
    if let Some((first, second)) = newparam{
        println!("first {:?}", first.to_string());
        println!("2nd {}", second);
        first.to_string()
    } else {
        println!("no delimt");
        "Nothing".to_string()
    }


}



pub async fn default(Path(param): Path<String>, Extension(db): Extension<Db>,) -> String {
    

    // let string_to_split = param.clone();

    // splitting at | then setting it as a iterator then unraping

    // have to do ts because borrow checker

    let ndpart = getnd(Path(param.clone()), Extension(db.clone())).await.to_string();
    println!("2nd part {:?}", ndpart);
    
    let firstpart = getfirst(Path(param.clone()), Extension(db.clone())).await.to_string();
    println!("first part {:?} ", firstpart);


    let name = firstpart.clone();
    let post = ndpart.clone();
    let yob = 2000;
    println!("inserted {} {}", name, ndpart);
    let insert_result = db
        .call(move |conn| {
            Ok(conn.execute(
                "INSERT INTO person (name, post,yob) VALUES (?1, ?2, ?3)",
                (&name, &post, &yob),
            )?)
        })
        .await;

    match insert_result {
        Ok(_) => format!(": {} 2nd{} ", param, ndpart),
        Err(e) => format!("DB error: {}", e),
    }

}


pub async fn db() -> Result<Db, Box<dyn std::error::Error>> {
    let db = Connection::open("my_db3").await?;
    db.call(|conn| {
        Ok(conn.execute(
            "CREATE TABLE IF NOT EXISTS person (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                post TEXT NOT NULL,
                yob  INTEGER,
                data  BLOB
            ) STRICT",
            (),
        )?)
    })
    .await?;
    Ok(Arc::new(db))
}

#[debug_handler]
pub async fn list_people(Extension(db): Extension<Db>) -> Json<Vec<Save>> {
    println!("listing people");
    let people = db
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT id, name,post,yob FROM person")?;
            let rows = stmt
                .query_map([], |row| {
                    Ok(Save { id: row.get(0)?, name: row.get(1)?, post: row.get(2)?, yob: row.get(3)?,})
             })?
            // collects the ok values into the vec and also leabs the room to propogate for errors
            .collect::<Result<Vec<_>, _>>()?;
                    
            Ok(rows)
        })
        .await
        .unwrap_or_default();

    Json(people)
   }

// db.call (|conn| {
// bleah
// })

pub async fn list_people2(Extension(db): Extension<Db>) -> String {
    let people = db
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT id, name,post, yob FROM person")?;
            let rows = stmt
                .query_map([], |row| {
                    Ok(Save {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        post: row.get(2)?,
                        yob: row.get(3)?,
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
                    .map(|p| format!("{}: {} {} ({})", p.id, p.name,p.post, p.yob))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
        Err(e) => format!("DB error: {}", e),
    }
}



pub async fn find(Path(param): Path<String>,Extension(db): Extension<Db>)  -> Json<Vec<Save>>{
    let result = list_people(Extension(db)).await;
    // println!("Result: {:?}", result);
    let search = param.clone();
    let find_thing: Vec<_> = 
    <Vec<Save> as Clone>::clone(&result)
    .into_iter()
    .filter(|wtf|wtf.name.contains(&search))
    .collect();

    println!(" for your search\n {:?}\n", find_thing);

    Json(find_thing) 
}

pub async fn find_name(Path(param): Path<String>, Extension(db): Extension<Db>) -> Json<Vec<Save>>  {

    let result = list_people(Extension(db)).await;
    let search = param.clone();
    println!(" searching for {}", search);
    let fine_thing: Vec<Save> = 
        <Vec<Save> as Clone>::clone(&result)
        .into_iter()
        .filter(|wtf| wtf.post.contains(&search))
        .collect();

    println!(" For your search\n {:?}", fine_thing);

    Json(fine_thing)
}
