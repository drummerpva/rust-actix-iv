mod model;
mod schema;
mod services;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:4000/");
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Database connected");
            pool
        }
        Err(error) => {
            println!("Database connection failed: {:?}", error);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .configure(services::config)
            .app_data(web::Data::new(AppState { db: pool.clone() }))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
