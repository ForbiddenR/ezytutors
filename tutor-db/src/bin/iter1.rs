// use std::{io, env};
// use dotenv::dotenv;

// use chrono::NaiveDateTime;
// use sqlx::PgPool;



// #[derive(Debug)]
// pub struct Course {
//     pub course_id: i32,
//     pub tutor_id: i32,
//     pub course_name: String,
//     pub posted_time: Option<NaiveDateTime>,
// }

// #[actix_rt::main]
// async fn main() -> io::Result<()> {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
//     let db_pool = PgPool::connect(&database_url).await.unwrap();
//     let course_rows = sqlx::query!(
//         r#"
//         -- DATABASE_URL=postgres://user:password@127.0.0.1:5432/ezytutors
//         select course_id, tutor_id, course_name, posted_time from ezy_course_c4 where course_id = $1"#,
//         1
//     )
//     .fetch_all(&db_pool)
//     .await
//     .unwarp();

// }
fn main() {
    
}