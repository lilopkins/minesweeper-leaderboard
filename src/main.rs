use chrono::prelude::*;
use rocket::{get, http::ContentType, launch, post, routes, Responder};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Database)]
#[database("leaderboard")]
struct Leaderboard(sqlx::SqlitePool);

#[derive(Responder)]
enum Error {
    #[response(status = 500)]
    SqlError(String),
}

impl From<rocket_db_pools::sqlx::Error> for Error {
    fn from(e: rocket_db_pools::sqlx::Error) -> Self {
        Self::SqlError(format!("{}", e))
    }
}

#[get("/")]
async fn index<'a>(mut db: Connection<Leaderboard>) -> Result<(ContentType, String), ()> {
    let mut lb = String::from(r#"
    <!doctype html>
    <html>
    <head>
    <title>Leaderboard</title>
    </head>
    <body style="margin: auto; max-width: 700px; padding: 0 1rem; font-family: 'Segoe UI', Roboto, sans-serif;">
    <h1>Leaderboard: Top 10 All-time</h1>
    <br /><ol>"#);

    if let Ok(rows) = sqlx::query("SELECT name, time FROM scores ORDER BY time ASC LIMIT 10")
        .fetch_all(&mut *db)
        .await {

        for entry in rows {
            lb.push_str(&format!(
                "<li>{} completed in {:.02} seconds</li>\n",
                entry.get::<String, &str>("name"),
                entry.get::<i64, &str>("time") as f32 / 1000f32
            ));
        }
    }
    lb.push_str("</ol></body>");
    Ok((ContentType::HTML, lb))
}

#[post("/submit/<name>/<time>")]
async fn submit<'a>(mut db: Connection<Leaderboard>, name: String, time: u32) -> Result<(ContentType, &'a str), Error> {
    sqlx::query("CREATE TABLE IF NOT EXISTS scores (
            name VARCHAR(255) NOT NULL,
            time INT NOT NULL,
            at INT NOT NULL);")
        .execute(&mut *db)
        .await?;
    sqlx::query("INSERT INTO scores (name, time, at) VALUES (?, ?, ?);")
        .bind(name)
        .bind(time)
        .bind(Utc::now().timestamp_millis())
        .execute(&mut *db)
        .await?;
    Ok((ContentType::Plain, "Your score has been submitted."))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Leaderboard::init())
        .mount("/", routes![index, submit])
}
