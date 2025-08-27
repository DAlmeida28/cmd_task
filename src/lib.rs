pub mod tasklib{

use std::fmt::{self};
use std::{fs::File};
use std::io::{BufReader, Write};
use clap::{Args, Parser, Subcommand, ValueEnum };
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands{
  Add(CLiTaskInput),
  List,
}

#[derive(Args, Debug, Clone)]
pub struct CLiTaskInput {
  #[arg(short = 't' , long)]
  pub title: Option<String>,
  #[arg(short = 'd', long)]
  pub description: Option<String>, 
  #[arg(short = 'u', long)]
  pub due_date: Option<String>,
  #[arg(short = 's', long)]
  pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
  pub id: String,
  pub title: String,
  pub due_date: String,
  pub description: String,
  pub status: Status, 
}

impl Task {

  }

#[derive(Serialize, Deserialize, Default)]
pub struct DB {
  pub tasks: Vec<Task>,
  #[serde(default)]
  pub counter: i32,
}

impl DB {
  pub fn add_task(&mut self, title: String, due_date: String, description: String, status: Status) {
    let gen_id = self.create_id();
    let task = Task {
        id: gen_id,
        title,
        due_date,
        description,
        status,
    };
    self.tasks.push(task);
    self.save_db();
    println!("Task saved successfully!");
    }

  pub fn list_tasks(&self) {
    println!("Tasks:");
    for task in &self.tasks {
        println!("id: {}, title: {}, description: {}, due-date: {}, status: {}", task.id, task.title, task.description, task.due_date, task.status);
        }
    }

  pub fn create_id(&mut self) -> String {
    let date = chrono::offset::Local::now().to_string();
    let date_slice = &date[5..10];
    let id = self.increment();
    date_slice.to_string() + id.as_str()
  }

 pub fn load_db() -> DB {
  if Path::new("db.json").exists() {
    let file = File::open("db.json").expect("failed to open db.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to read new file")
    } else {
      DB::default()
    }
  }

  pub fn save_db(&self) {
    let json = serde_json::to_string_pretty(self).expect("Failed to serialize database");
    let mut file = File::create("db.json").expect("Failed to write to db.json");
    file.write_all(json.as_bytes()).expect("failed to write JSON to db.json");
  }

  pub fn increment(&mut self) -> String {
    self.counter +=1;
    self.save_db();
    self.counter.to_string()
  }
}


#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize,)]
pub enum Status {
  Open,
  Pending,
  Completed,
}

impl fmt::Display for Status {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let status = match self {
      Status::Open => "Open",
      Status::Pending => "Pending",
      Status::Completed => "Completed"
    };
    write!(f, "{}", status)
  }
  }
  }

