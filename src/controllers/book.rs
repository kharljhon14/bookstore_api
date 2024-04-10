use std::time::SystemTime;

use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::{
    prelude::DateTimeUtc, ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait,
    QueryOrder, Set,
};

use crate::{
    auth::AuthenticatedUser,
    entities::{book, prelude::*},
};

use super::{ErrorResponse, GenericResponse, Response, SuccessResponse};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBook {
    id: i32,
    title: String,
    year: String,
    cover: String,
    author_id: i32,
}

impl From<&book::Model> for ResBook {
    fn from(b: &book::Model) -> Self {
        Self {
            id: b.id,
            title: b.title.to_owned(),
            year: b.year.to_owned(),
            cover: b.cover.to_owned(),
            author_id: b.author_id,
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBookList {
    total: usize,
    books: Vec<ResBook>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqBook {
    author_id: i32,
    title: String,
    year: String,
    cover: String,
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
        .map(ResBook::from)
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBookList {
            total: books.len(),
            books,
        }),
    )))
}

#[post("/", data = "<req_book>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_book: Json<ReqBook>,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = book::ActiveModel {
        user_id: Set(user.id as i32),
        author_id: Set(req_book.author_id),
        title: Set(req_book.title.to_owned()),
        year: Set(req_book.year.to_owned()),
        cover: Set(req_book.cover.to_owned()),
        ..Default::default()
    };

    let book = book.insert(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResBook::from(&book)),
    )))
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = Book::find_by_id(id).one(db).await?;

    let book = match book {
        Some(b) => b,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                Json(GenericResponse {
                    message: "Cannot find a book with specified ID".to_string(),
                }),
            )))
        }
    };

    Ok(SuccessResponse((Status::Ok, Json(ResBook::from(&book)))))
}

#[put("/<id>", data = "<req_book>")]
pub async fn update(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
    req_book: Json<ReqBook>,
) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = Book::find_by_id(id).one(db).await?;

    let mut book: book::ActiveModel = match book {
        Some(b) => b.into(),
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                Json(GenericResponse {
                    message: "Cannot find book with specified ID".to_string(),
                }),
            )))
        }
    };

    book.author_id = Set(req_book.author_id.to_owned());
    book.title = Set(req_book.title.to_owned());
    book.year = Set(req_book.year.to_owned());
    book.cover = Set(req_book.cover.to_owned());

    book.updated_at = Set(Some(DateTimeUtc::from(SystemTime::now())));

    let book = book.update(db).await?;

    Ok(SuccessResponse((Status::Ok, Json(ResBook::from(&book)))))
}

#[delete("/<id>")]
pub async fn delete(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
    id: i32,
) -> Response<Json<GenericResponse>> {
    let db = db as &DatabaseConnection;

    let book = Book::find_by_id(id).one(db).await?;

    let book = match book {
        Some(b) => b,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                Json(GenericResponse {
                    message: "Cannot find book with specified ID".to_string(),
                }),
            )))
        }
    };

    book.delete(db).await?;

    Ok(SuccessResponse((
        Status::Ok,
        Json(GenericResponse {
            message: "Book deleted".to_string(),
        }),
    )))
}
