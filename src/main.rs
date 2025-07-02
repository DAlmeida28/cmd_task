use clap::Parser;
use cmd_task::tasklib::{Task};

fn main() {
Task::check_taskdb();       
    
    let task_arg = Task::parse();
    println!("{:?}", task_arg);



}