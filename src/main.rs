use chrono::prelude::*;
use rocket::{get, http::ContentType, launch, post, routes};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Database)]
#[database("leaderboard")]
struct Leaderboard(sqlx::SqlitePool);

#[get("/")]
async fn index<'a>(mut db: Connection<Leaderboard>) -> Result<(ContentType, String), ()> {
    let mut lb = String::from("<!doctype html><html><head><title>Leaderboard</title></head><body><h1>Leaderboard</h1><br /><ol>");

    let rows = sqlx::query("SELECT name, time FROM scores ORDER BY time ASC LIMIT 10")
        .fetch_all(&mut *db)
        .await
        .unwrap();
    for entry in rows {
        lb.push_str(&format!(
            "<li>{} completed in {:.02} seconds</li>\n",
            entry.get::<String, &str>("name"),
            entry.get::<i64, &str>("time") / 1000
        ));
    }
    lb.push_str("</ol></body>");
    Ok((ContentType::HTML, lb))
}

#[post("/submit/<name>/<time>")]
async fn submit<'a>(mut db: Connection<Leaderboard>, name: String, time: u32) -> Result<(ContentType, &'a str), ()> {
    sqlx::query("CREATE TABLE IF NOT EXISTS scores (
            name VARCHAR(255) NOT NULL,
            time INT NOT NULL,
            at INT NOT NULL);")
        .execute(&mut *db)
        .await
        .unwrap();
    sqlx::query("INSERT INTO scores (name, time, at) VALUES (?, ?, ?);")
        .bind(name)
        .bind(time)
        .bind(Utc::now().timestamp_millis())
        .execute(&mut *db)
        .await
        .unwrap();
    Ok((ContentType::Plain, "Your score has been submitted."))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Leaderboard::init())
        .mount("/", routes![index, submit])
}
