#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub content: String,
}

impl Clone for Todo {
    fn clone(&self) -> Self {
        return Todo{id:self.id, content:self.content.clone()};
    }
}