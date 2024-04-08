use std::sync::Mutex;
use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>, members: web::Data<Mutex<Vec<Member>>>) -> impl Responder {
    let members = members.lock().unwrap();
    members.iter()
        .find(|member| member.name == *name)
        .map(|member| Ok(web::Json(format!("Hello {name}!, your age is {age}!", name = member.name, age = member.age))))
        .unwrap_or_else(|| Err(actix_web::error::ErrorNotFound("Member not found")))
}

#[derive(Clone, Serialize)]
struct Member {
    name: String,
    age: u8,
}

#[get("/members")]
async fn get_members(members: web::Data<Mutex<Vec<Member>>>) -> Result<impl Responder> {
    let members = members.lock().unwrap();
    Ok(web::Json(members.clone()))
}

#[get("/members/{index}")]
async fn get_member_by_index(
    index: web::Path<usize>,
    members: web::Data<Mutex<Vec<Member>>>,
) -> Result<impl Responder, actix_web::Error> {
    let members = members.lock().unwrap();
    members
        .get(*index)
        .map(|member| Ok(web::Json(member.clone())))
        .unwrap_or_else(|| Err(actix_web::error::ErrorNotFound("Member not found")))
}

#[get("/members/{name}/age")]
async fn get_member_age_by_name(
    name: web::Path<String>,
    members: web::Data<Mutex<Vec<Member>>>,
) -> Result<impl Responder, actix_web::Error> {
    let members = members.lock().unwrap();
    members
        .iter()
        .find(|member| member.name == *name)
        .map(|member| Ok(web::Json(member.age)))
        .unwrap_or_else(|| Err(actix_web::error::ErrorNotFound("Member not found")))
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let members = vec![
        Member {
            name: "Alice".to_string(),
            age: 20,
        },
        Member {
            name: "Bob".to_string(),
            age: 22,
        },
        Member {
            name: "Charlie".to_string(),
            age: 25,
        },
    ];

    let members = web::Data::new(Mutex::new(members));

    HttpServer::new(move || 
        App::new()
            .service(greet)
            .service(get_members)
            .service(get_members)
            .service(get_member_by_index)
            .service(get_member_age_by_name)
            .app_data(members.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
