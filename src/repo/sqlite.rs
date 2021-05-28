use anyhow::Result;
use rusqlite::{params, Connection};

use super::super::model::Todo;
use super::interface::Repo;

pub struct SqliteRepo {
    conn: Connection,
}

impl SqliteRepo {
    pub fn new() -> Result<SqliteRepo> {
        let conn = Connection::open("data.sqlite")?;
        return Ok(SqliteRepo{conn:conn});
    }
}

impl Repo for SqliteRepo {
    fn get_all(&self) -> Result<Vec<Todo>> {
        let mut stmt = self.conn.prepare("SELECT * FROM todos").expect("SELECT SQL failed");
        let todo_iter = stmt.query_map([],|row|{
            Ok(Todo{
                id: row.get(0)?,
                content: row.get(1)?,
            })
        }).expect("SELECT SQL failed");
        let mut todos: Vec<Todo> = Vec::new();
        for todo in todo_iter {
            let t = &todo.unwrap();
            todos.push(t.clone());
        }
        return Ok(todos);
    }

    fn get(&self, id:i32) -> Option<Todo>{
        match self.get_all() {
            Err(e) => {
                println!("get_all() failed; {}", e);
                return None;
            },
            Ok(todos) => {
                for todo in todos {
                    if todo.id == id {
                        return Some(todo);
                    }                    
                }
            },
        }
        return None;
    }

    fn set(&mut self, todo:Todo){
        unimplemented!();
    }

    fn create(&mut self, content:&str) -> Result<()> {
        match self.conn.execute(
            "INSERT INTO todos (content) VALUES (?1)",
            params![content],
        ){
            Ok(_) => {return Ok(())},
            Err(e) => {return Err(e.into())},
        }
    }

    fn delete(&mut self, id:i32){
        unimplemented!();
    }
}
