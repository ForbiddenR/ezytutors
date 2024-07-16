// Environment imports
use dotenv::dotenv;
use std::env;
// Standard lib imports
use std::io;
// SQLx imports

use sqlx::postgres::PgPool;
//Utilities
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    println!("database_url {}", database_url);
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    println!("is closed {}", db_pool.is_closed());
    sqlx::query!("").fetch_all(&db_pool)
    .await
    .unwrap();
    
    Ok(())
}