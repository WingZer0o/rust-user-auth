use actix_web::{delete, get, post, HttpResponse, Responder};

#[get("/users")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("hello from users")
}

#[get("/users/{id}")]
pub async fn get_user_by_id() -> impl Responder {
    HttpResponse::Ok().body("hello from get users by id")
}

#[post("/users")]
pub async fn add_user() -> impl Responder {
    HttpResponse::Ok().body("hello from add users")
}

#[delete("/users/{id}")]
pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("hello from delete users")
}