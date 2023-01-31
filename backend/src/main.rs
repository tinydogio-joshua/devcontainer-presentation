use axum::{extract::State, response::Html, routing::get, Router};
use sqlx::{postgres::PgPoolOptions, Error as SQLxError, PgPool};
use std::net::SocketAddr;

async fn db() -> Result<PgPool, SQLxError> {
    let connection_string = "postgres://postgres:postgres@localhost:5432/postgres";

    PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await
}

async fn index_handler(State(pool): State<PgPool>) -> Html<String> {
    let views = update_current_count(&pool).await;
    match views {
        Ok(current) => Html(format!("<h1>Random Number: {}</h1>", current)),
        Err(_) => Html(String::from("🐼 Unable To Get Current Views")),
    }
}

async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("🚨 Unable to Run Migrations");
    println!("✅ Migrations Complete");
}

async fn run_server(pool: PgPool) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/", get(index_handler))
        .with_state(pool);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn update_current_count(pool: &PgPool) -> Result<i32, SQLxError> {
    sqlx::query(
        "
        UPDATE counter
        SET current = current + 1;
    ",
    )
    .execute(pool)
    .await?;

    let counter = sqlx::query!(
        "
        SELECT current
        FROM COUNTER;
    "
    )
    .fetch_one(pool)
    .await?;

    Ok(counter.current)
}

#[tokio::main]
async fn main() {
    match db().await {
        Ok(pool) => {
            run_migrations(&pool).await;
            run_server(pool).await;
        }
        Err(_) => println!("🚨 Unable To Connect To Database"),
    }
}
