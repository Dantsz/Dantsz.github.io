use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Article
{
    pub name : String,
}