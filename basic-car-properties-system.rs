struct Car {
  mpg: i64,
  color: String,
  top_speed: i32,
}

impl Car {
  fn set_mpg(&mut self, new_value: i64) {
    self.mpg = new_value;
  }
  fn set_color(&mut self, new_value: String) {
    self.color = new_value.to_string();
  }
  fn set_top_speed(&mut self, new_value: i32) {
    self.top_speed = new_value;
  }
}

fn main() {
  let mut newCar = Car{mpg: 100, color: "Green".to_string(), top_speed: 100};
  println!("This car has {} MPG, color of it is {} and the top speed of it is {}", newCar.mpg, newCar.color, newCar.top_speed);
  newCar.set_mpg(150);
  newCar.set_color("Red".to_string());
  newCar.set_top_speed(170);
  println!("This car has {} MPG, color of it is {} and the top speed of it is {}", newCar.mpg, newCar.color, newCar.top_speed);
}
