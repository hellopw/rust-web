use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use crate::entity::person::Person;
// use crate::dao::person_dao::{DdlOp, DmlOp};

// use PersonDao
mod dao;
mod entity;
mod service;
mod myerror;


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(service::person_service::test_url_path)
            .service(service::person_service::test_url_param)
            .service(service::person_service::test_web_body)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

