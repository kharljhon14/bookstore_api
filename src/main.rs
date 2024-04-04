#[macro_use]
extern crate rocket;

use migrator::{Migrator, MigratorTrait};

mod db;
mod entities;
mod migrator;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("BOOKSTORE_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("BOOKSTORE_DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("BOOKSTORE_DB_USERNAME").unwrap_or("root".to_string()),
            db_password: std::env::var("BOOKSTORE_DB_PASSWORD")
                .unwrap_or("@Password123".to_string()),
            db_database: std::env::var("BOOKSTORE_DB_DATABASE").unwrap_or("bookstore".to_string()),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let config = AppConfig::default();

    let db = db::connect(&config).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    rocket::build().mount("/", routes![index])
}
