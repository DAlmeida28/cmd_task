// use chrono::Date;
use clap::Parser;
use cmd_task::tasklib::*;

fn main() {

  let cli = Cli::parse();

  match cli.command {
    Commands::Add(input) => {
      Task::add_task(input.title.expect("No title Provided."), input.description.expect("No Description provided."), input.due_date.expect("No due date provided."), input.status );
    }
    Commands::List => { Task::list_task(); }
  }
}