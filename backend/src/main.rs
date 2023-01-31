use axum::{response::Html, routing::get, Router};
use sqlx::{postgres::PgPoolOptions, Error as SQLxError, PgPool};
use std::net::SocketAddr;

async fn db() -> Result<PgPool, SQLxError> {
    let connection_string = "postgres://postgres:postgres@localhost:5432/postgres";

    PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await
}

async fn index_handler() -> Html<&'static str> {
    Html("<h1>Hello, World</h1>")
}

async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("🚨 Unable to Run Migrations");
    println!("✅ Migrations Complete");
}

async fn run_server() {
    let app = Router::new().route("/", get(index_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

#[tokio::main]
async fn main() {
    match db().await {
        Ok(pool) => {
            run_migrations(&pool).await;
            run_server().await;
        }
        Err(_) => println!("🚨 Unable To Connect To Database"),
    }
}
