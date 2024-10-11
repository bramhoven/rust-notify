use deadpool_diesel::postgres::{Runtime, Manager, Pool};

pub fn establish_connection(database_url: &String) -> Pool {
    let manager = Manager::new(database_url, Runtime::Tokio1);
    
    Pool::builder(manager)
        .max_size(8)
        .build()
        .unwrap()
}
