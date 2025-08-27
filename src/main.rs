use clap::Parser;
use cmd_task::tasklib::*;

fn main() {
  let cli = Cli::parse();

  match cli.command {
      Commands::Add(input) => {
          let mut db = DB::load_db();  // load DB
          db.add_task(
              input.title.expect("No title provided."),
              input.description.expect("No description provided."),
              input.due_date.expect("No due date provided."),
              input.status,
          );
      }
      Commands::List => {
          let db = DB::load_db();      // load DB
          db.list_tasks();
      }
  }
}