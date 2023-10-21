use std::collections::HashMap;
use actix_web::{get, post, Responder, web};
use rusqlite::{Connection, Error};
use crate::dao::person_dao::{DmlOp, PersonDao};
use crate::entity::person::Person;
use crate::myerror::MyError;

pub struct PersonService<'a> {
    dao: &'a PersonDao<'a>,
}

pub trait PersonTrait {
    fn get_all(&self) -> Result<Vec<Person>, Error> {
        Ok(vec![])
    }
}

impl PersonTrait for PersonService<'_> {
    fn get_all(&self) -> Result<Vec<Person>, Error> {
        todo!()
    }
}

#[get("/person/{name}")]
async fn test_url_path(name: web::Path<String>) -> Result<impl Responder, MyError> {
    let conn = Connection::open("a.db")?;
    let person_dao1 = PersonDao::new(&conn);
    let t = person_dao1.get_by_name(name.to_string())?;
    Ok(web::Json(t))
}

/// 测试参数格式 /person?id=1
///
/// # Arguments
///
/// * `map`:
///
/// returns: Result<impl Responder<Body=<unknown>>+Sized, MyError>
///
/// # Examples
///
/// ```
///
/// ```
#[get("/person")]
async fn test_url_param(map: web::Query<HashMap<String, i32>>) -> Result<impl Responder, MyError> {
    let conn = Connection::open("a.db")?;
    let person_dao1 = PersonDao::new(&conn);
    let t = person_dao1.get_by_id(*map.get(&*"id".to_string()).unwrap())?;
    Ok(web::Json(t))
}

#[post("/person")]
async fn test_web_body(person: web::Json<Person>) -> Result<impl Responder, MyError> {
    let conn = Connection::open("a.db")?;
    let person_dao1 = PersonDao::new(&conn);
    person_dao1.insert(&person)?;
    Ok(web::Json(person))
}