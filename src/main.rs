use clap::Parser;
use cmd_task::tasklib::{Task, DB};

fn main() {
    DB::check_taskdb();       
    
    let task_arg = Task::parse();

    if let Err(e) = task_arg.save() {
        eprintln!("Error saving task: {}", e);
    } else {
        println!("Task Saved!");
    }

}