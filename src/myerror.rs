use std::fmt;
use actix_web::{HttpResponse, ResponseError, web};
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Debug)]
#[allow(dead_code)]
pub enum MyError {
    SQLError(rusqlite::Error),
    SQLError1(String),
    ActixError(String),
    BussError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::SQLError(e) => {
                println!("Database error occurred: {:?}", e);
                e.to_string().into()
            }
            MyError::SQLError1(e) => {
                println!("Database error occurred: {:?}", e);
                e.into()
            }
            MyError::ActixError(e) => {
                println!("Database error occurred: {:?}", e);
                e.into()
            }
            MyError::BussError(e) => {
                println!("buss error occurred: {:?}", e);
                e.into()
            }
        }
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::SQLError(_e) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::SQLError1(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BussError(_msg) => StatusCode::OK,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}


impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.error_response())
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}


impl From<rusqlite::Error> for MyError {
    fn from(err: rusqlite::Error) -> Self {
        MyError::SQLError1(err.to_string())
    }
}