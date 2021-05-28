use repo::interface::Repo;
use model::Todo;

mod model;
mod repo;

fn memory(){
    println!("--- memory ---");
    let mut repo = repo::memory::MemoryRepo::new();
    let todos = repo.get_all();
    for todo in todos {
        println!("{:?}", todo);
    }
    let todo = repo.get(1);
    println!("{:?}", todo);
    repo.set(Todo{id:2, content:"charlie".to_string()});
    repo.delete(2);
    let todos = repo.get_all();
    for todo in todos {
        println!("{:?}", todo);
    }
}

fn sqlite(){
    println!("--- sqlite ---");
    let mut repo = repo::sqlite::SqliteRepo::new().expect("failed to initialize SqliteRepo");                                                                                                                                                                                                                                                                                                                                                                                    
    let todos = repo.get_all();
    for todo in todos {
        println!("{:?}", todo);
    }
    let todo = repo.get(1);
    println!("{:?}", todo);
}

fn main() {
    memory();
    sqlite();
}
