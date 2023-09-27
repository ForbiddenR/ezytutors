use std::{io, sync::Mutex, vec};

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../state.rs"]
mod state;
#[path = "../routes.rs"]
mod routes;

use actix_web::{web, App, HttpServer};
use routes::{general_routes, course_routes};
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await
}
