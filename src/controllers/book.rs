use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
    State,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};

use crate::{
    auth::AuthenticatedUser,
    entities::{book, prelude::*},
};

use super::{Response, SuccessResponse};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBook {
    id: i32,
    title: String,
    year: String,
    cover: String,
    author_id: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBookList {
    total: usize,
    books: Vec<ResBook>,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<ResBookList>> {
    let db = db as &DatabaseConnection;

    let books = Book::find()
        .order_by_desc(book::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(|book| ResBook {
            id: book.id,
            author_id: book.author_id,
            title: book.title.to_owned(),
            year: book.year.to_owned(),
            cover: book.cover.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBookList {
            total: books.len(),
            books,
        }),
    )))
}

#[post("/")]
pub async fn create() -> Response<String> {
    todo!()
}

#[get("/<id>")]
pub async fn show(id: u32) -> Response<String> {
    todo!()
}

#[put("/<id>")]
pub async fn update(id: u32) -> Response<String> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete(id: u32) -> Response<String> {
    todo!()
}
