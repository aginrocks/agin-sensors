mod connector;
pub mod database;
mod databases;
pub mod global_config;
mod project_config;
mod state;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
