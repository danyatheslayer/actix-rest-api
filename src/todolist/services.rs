use super::models::{AppState, CreateEntryData, TodoList, UpdateEntryData};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use sqlx;

#[get("/todolist/entries")]
async fn get_entries(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(TodoList, "SELECT * FROM todolist")
        .fetch_all(&state.db)
        .await
    {
        Ok(todolist) => HttpResponse::Ok().json(todolist),
        Err(_) => HttpResponse::NotFound().json("Sometheing happend due get"),
    }
}

#[post("/todolist/entries/{id}")]
async fn post_entries(
    state: Data<AppState>,
    path_id: Path<i32>,
    data: Json<CreateEntryData>,
) -> impl Responder {
    let id: i32 = path_id.into_inner();

    match sqlx::query_as!(
        TodoList,
        "INSERT INTO todolist (id, title, date) VALUES ($1, $2, $3) RETURNING id, title, date",
        id,
        data.title.clone(),
        data.date
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(todolist) => HttpResponse::Ok().json(todolist),
        Err(_) => HttpResponse::NotFound().json("Sometheing happend due post"),
    }
}

#[put("/todolist/entries/{id}")]
async fn update_entries(
    state: Data<AppState>,
    path_id: Path<i32>,
    data: Json<UpdateEntryData>,
) -> impl Responder {
    let id = path_id.into_inner();

    match sqlx::query_as!(
        TodoList,
        "UPDATE todolist SET title = $1 WHERE id = $2 RETURNING id, title, date",
        data.title.clone(),
        id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(todolist) => HttpResponse::Ok().json(todolist),
        Err(_) => HttpResponse::NotFound().json("Sometheing happend due get"),
    }
}

#[delete("/todolist/entries/{id}")]
async fn delete_entries(state: Data<AppState>, path_id: Path<i32>) -> impl Responder {
    let id = path_id.into_inner();

    match sqlx::query_as!(
        TodoList,
        "DELETE FROM todolist WHERE id = $1 RETURNING id, title, date",
        id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(todolist) => HttpResponse::Ok().json(todolist),
        Err(_) => HttpResponse::NotFound().json("Sometheing happend due get"),
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_entries)
        .service(post_entries)
        .service(delete_entries)
        .service(update_entries);
}
