pub mod tasklib{
use std::fs;
use clap::{Parser, ValueEnum };
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Task {
  id: u32,
  pub title: String,
  pub due_date: String,
  pub description: String,
  pub status: Status, 
}

pub struct DB {
  filepath: String
}

impl Task {
  pub fn new_task(title: String, description: String, due_date: char) {
    println!("This called the function with {}, {}, {}", title, description, due_date);
  }
  pub fn check_taskdb() {
    let db_file = fs::metadata("../db.json").is_ok();
    match db_file{
      true => println!("db Found"),
      false => println!("db not found"),
    }

    }
}



#[derive(Debug, Clone, ValueEnum)]
pub enum Status {
  Open,
  Pending,
  Completed,
}
}
