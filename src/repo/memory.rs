use std::collections::HashMap;

use anyhow::Result;

use super::super::model::Todo;
use super::interface::Repo;

pub struct MemoryRepo {
    todos: HashMap<i32, Todo>,
    new_id: i32,
}

impl MemoryRepo {
    pub fn new() -> MemoryRepo {
        let mut todos = HashMap::new();
        todos.insert(
            1,
            Todo {
                id: 1,
                content: "alpha".to_string(),
            },
        );
        todos.insert(
            2,
            Todo {
                id: 2,
                content: "bravo".to_string(),
            },
        );
        return MemoryRepo { todos: todos , new_id:3};
    }
}

impl Repo for MemoryRepo {
    fn get_all(&self) -> Result<Vec<Todo>> {
        let mut results: Vec<Todo> = Vec::new();
        for (_id, td) in &self.todos {
            results.push((*td).clone());
        }
        return Ok(results);
    }

    fn get(&self, id:i32) -> Option<Todo>{
        if let Some(td) = self.todos.get(&id){
            return Some(td.clone());
        }
        return None;
    }

    fn set(&mut self, todo:Todo){
        self.todos.insert(todo.id, todo);
    }
 
    fn create(&mut self, content:&str) -> Result<()> {
        self.todos.insert(self.new_id, Todo{id:self.new_id, content:content.to_string()});
        self.new_id += 1;
        return Ok(());
    }

    fn delete(&mut self, id:i32){
        self.todos.remove(&id);
    }
}
