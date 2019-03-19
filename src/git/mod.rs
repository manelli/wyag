pub mod repository;
use git::repository::Repository;
use std::env;

pub fn init(path: Option<&str>) {
    let cd = env::current_dir().unwrap();
    let p = match path {
        Some(p) => p,
        _ => cd.to_str().unwrap(),
    };

    Repository::create(p);
    println!("Initialized empty Git repository in {}", p);
}
