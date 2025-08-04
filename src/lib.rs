pub mod tasklib{

use std::fmt::{self};
use std::{fs::File};
use std::io::{BufReader, Write};
use clap::{Args, Parser, Subcommand, ValueEnum };
use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::create_id;


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
  #[arg(short, long)]
  pub title: Option<String>,
  #[arg(short, long)]
  pub description: Option<String>, 
  #[arg(short = 'u', long)]
  pub due_date: Option<String>,
  #[arg(short, long)]
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
  pub fn add_task (title: String, due_date: String, description: String, status: Status){
    let mut ldb = DB::load_db();
    let id = create_id();
    let task = Task { 
      id,
      title,
      due_date,
      description,
      status
    };

    ldb.tasks.push(task);
    DB::save_db(&ldb);
    println!("Task saved succesfully!");
   }

  pub fn list_task() {
    let ldb = DB::load_db();
    println!("Tasks: ");
    for task in ldb.tasks {
      println!("id: {}, title: {}, description: {}, due-date: {}, status: {}", task.id, task.title, task.description, task.due_date, task.status)
    }
  }

  pub fn gen_id() {

  }
  }

#[derive(Serialize, Deserialize, Default)]
pub struct DB {
  pub tasks: Vec<Task>,
}

impl DB {
 pub fn load_db() -> DB {
  if Path::new("db.json").exists() {
    let file = File::open("db.json").expect("failed to open db.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to read new file")
    } else {
      DB::default()
    }
  }

  pub fn save_db(db: &DB) {
    let json = serde_json::to_string_pretty(db).expect("Failed to serialize database");
    let mut file = File::create("db.json").expect("Failed to write to db.json");
    file.write_all(json.as_bytes()).expect("failed to write JSON to db.json");
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

fn create_id() -> String {
  let date = chrono::offset::Local::now().to_string();
  let date_slice = &date[5..10];

  let mut id: i32 = 1;
  if id >= 1 {
    id += 1;
  }
  
  let id_string: String = date_slice.to_string() + &id.to_string();
  id_string
}