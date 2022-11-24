use actix_web::{get, web::Data, App, HttpServer,};
mod todolist;
use todolist::models::AppState;
use todolist::services;
use sqlx::postgres::PgPoolOptions;


#[get("/")]
async fn index() -> String {
    format!("test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = std::env::var("DATABASE_URL").expect("not found");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("DB connection pool ERR");
 

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {db: pool.clone()}))
            .service(index)
            .configure(services::config)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}