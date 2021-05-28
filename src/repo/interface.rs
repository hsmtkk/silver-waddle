use anyhow::Result;

use super::super::model::Todo;

pub trait Repo {
    fn get_all(&self) -> Result<Vec<Todo>>;
    fn get(&self, id:i32) -> Option<Todo>;
    fn set(&mut self, todo:Todo);
    fn create(&mut self, content:&str) -> Result<()>;
    fn delete(&mut self, id:i32);
}
