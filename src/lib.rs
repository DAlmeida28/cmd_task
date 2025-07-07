pub mod tasklib{

use std::{fs, fs::File};
use std::io::{Write, Result};
use clap::{Parser, ValueEnum };
use serde::Serialize;

#[derive(Parser, Debug, Serialize)]
#[command(version, about, long_about = None)]
pub struct Task {
  id: u32,
  pub title: String,
  pub due_date: String,
  pub description: String,
  pub status: Status, 
}

impl Task {
  pub fn save(&self) -> std::io::Result<()>{
    let tojson = serde_json::to_string(self)
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut tofile = std::fs::File::options()
      .append(true)
      .create(true)
      .open("db.json")
      .map_err(|e| std::io::Error::new(e.kind(), "Could not open DB file"))?;

    writeln!(tofile, "{}", tojson)
      .map_err(|e| std::io::Error::new(e.kind(), "Could not write to DB file"))?;

    Ok(())
  }
}

pub struct DB {
  filepath: String
}

impl DB {
  pub fn check_taskdb() {
    let db_file = fs::metadata("db.json").is_ok();
      match db_file{
        true => println!("db Found"),
        false => {let db = File::create("db.json");},
  }
 }
}


#[derive(Debug, Clone, ValueEnum, Serialize)]
pub enum Status {
  Open,
  Pending,
  Completed,
}
}
