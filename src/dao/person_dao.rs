use rusqlite::Connection;
use crate::myerror::MyError;
use crate::Person;

#[derive(Debug)]
pub struct PersonDao<'a> {
    conn: &'a Connection,
}

pub trait DdlOp {
    fn create(&self) -> Result<i32, MyError>;
}

pub trait DmlOp {
    fn insert(&self, person: &Person) -> Result<i32, MyError>;
    fn update(&self, person: Person) -> Result<i32, MyError>;
    fn delete(&self, id: i32) -> Result<i32, MyError>;
    fn get_all(&self) -> Result<Vec<Person>, MyError>;
    fn get_by_id(&self, id: i32) -> Result<Person, MyError>;
    fn get_by_name(&self, name: String) -> Result<Person, MyError>;
}

impl PersonDao<'_> {
    pub fn new(conn: &Connection) -> PersonDao {
        PersonDao { conn }
    }
}

impl DdlOp for PersonDao<'_> {
    fn create(&self) -> Result<i32, MyError> {
        self.conn.execute(
            "CREATE TABLE person (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                data  BLOB
            )",
            (),
        )?;
        Ok(1)
    }
}

impl DmlOp for PersonDao<'_> {
    fn insert(&self, person: &Person) -> Result<i32, MyError> {
        self.conn.execute(
            "INSERT INTO person (name, data) VALUES (?1, ?2)",
            (&person.name, &person.data),
        )?;
        Ok(1)
    }
    fn update(&self, person: Person) -> Result<i32, MyError> {
        self.conn.execute(
            "UPDATE person SET name = ?1, data = ?2 WHERE id = ?3",
            (&person.name, &person.data, person.id),
        )?;
        Ok(1)
    }
    fn delete(&self, id: i32) -> Result<i32, MyError> {
        self.conn.execute("DELETE FROM person WHERE id = ?1", (id, ))?;
        Ok(1)
    }

    fn get_all(&self) -> Result<Vec<Person>, MyError> {
        let mut stmt = self.conn.prepare("SELECT id, name, data FROM person ")?;
        let person_iter = stmt.query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })?;
        let mut res = Vec::new();
        for person in person_iter {
            res.push(person.unwrap())
        }
        Ok(res)
    }

    fn get_by_id(&self, id: i32) -> Result<Person, MyError> {
        let mut stmt = self.conn.prepare("SELECT id, name, data FROM person WHERE id = ?1")?;
        let person_iter = stmt.query_map([id], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })?;
        for person in person_iter {
            return Ok(person.unwrap());
        }
        Err(MyError::BussError("cannot find person by id ".to_string()))
    }
    fn get_by_name(&self, name: String) -> Result<Person, MyError> {
        let mut stmt = self.conn.prepare("SELECT id, name, data FROM person WHERE name = ?1")?;
        let person_iter = stmt.query_map([name], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })?;
        for person in person_iter {
            return Ok(person.unwrap());
        }
        Err(MyError::BussError("cannot find person by name ".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;
    use crate::dao::person_dao;
    use crate::dao::person_dao::{DdlOp, DmlOp};
    use crate::entity::person::Person;
    use crate::myerror::MyError;

    #[test]
    fn test_dao() -> Result<(), MyError> {
        // let conn = Connection::open("a.db")?;
        let conn = Connection::open_in_memory()?;

        let person_dao1 = person_dao::PersonDao::new(&conn);

        let t = person_dao1.create();
        println!("create res: {:?}", t);

        let r = person_dao1.insert(&Person {
            id: 1,
            name: "binwang".to_string(),
            data: Some(Vec::new()),
        })?;
        println!("insert res: {:?}", r);

        let t = person_dao1.get_by_name("binwang".to_string())?;
        println!("get by name res: {:?}", t);

        let t = person_dao1.get_by_id(1)?;
        println!("get by id res: {:?}", t);

        let t = person_dao1.get_all()?;
        println!("get all: {:?}", t);

        Ok(())
    }
}