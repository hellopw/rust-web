use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub(crate) name: String,
    pub(crate) data: Option<Vec<u8>>,
}
