use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: u8,
    email: String,
}

async fn get_filtered_users() -> impl Responder {
    match read_and_filter_users("users.csv", 18) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

fn read_and_filter_users(file_path: &str, min_age: u8) -> Result<Vec<User>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut users = Vec::new();

    for result in rdr.deserialize() {
        let user: User = result?;
        if user.age >= min_age {
            users.push(user);
        }
    }
    Ok(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/users", web::get().to(get_filtered_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
