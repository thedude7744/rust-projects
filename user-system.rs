struct User {
  username: String,
  membership: String,
  age: u32,
  post_amount: u32,
  thread_amount: u32,
  reputation: u16,
  is_active: bool,
}

impl User {
  fn increase(&mut self, value: &str, increase_value: u32) {
    match value {
      "age" => self.age += increase_value,
      "post_amount" => self.post_amount += increase_value,
      "thread_amount" => self.thread_amount += increase_value,
      _ => panic!("Invalid field name"),
    }
  }
  fn decrease(&mut self, value: &str, decrease_value: u32) {
    match value {
      "age" => self.age -= decrease_value,
      "post_amount" => self.post_amount -= decrease_value,
      "thread_amount" => self.thread_amount -= decrease_value,
      _ => panic!("Invalid field name"),
    }
  }
  fn change_username(&mut self, value: String) {
    self.username = value;
  }
  fn change_activity(&mut self, value: bool) {
    self.is_active = value;
  }
}

fn main() {
  // example
  let mut newUser = User {username: "John Doe".to_string(), membership: "Premium".to_string(), age: 0, post_amount: 0, thread_amount: 0, reputation: 0, is_active: true};
  newUser.increase("age", 1);
  newUser.increase("post_amount", 3);
  println!("This user is named {} and is {} years old. They have {} posts and {} threads. They have a reputation of {} and their account is {}.", newUser.username, newUser.age, newUser.post_amount, newUser.thread_amount, newUser.reputation, newUser.is_active);
  newUser.decrease("post_amount", 1);
  newUser.change_username("Banned".to_string());
  newUser.change_activity(false);
  println!("This user is named {} and is {} years old. They have {} posts and {} threads. They have a reputation of {} and their account is {}.", newUser.username, newUser.age, newUser.post_amount, newUser.thread_amount, newUser.reputation, newUser.is_active);
}
