pub struct Task {
  id: u32,
  pub title: String,
  pub due_date: char,
  pub description: String,
  pub status: Status, 
}

impl Task {
  pub fn new_task(title: String, description: String, due_date: char) {
    println!("This called the function with {}, {}, {}", title, description, due_date);
  }

}

pub enum Status {
  Open,
  Pending,
  Completed,
}

