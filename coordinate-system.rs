#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Movement {
    direction: Direction,
    steps: u8,
}

struct Coordinates {
    x: i8,
    y: i8,
}

impl Direction {
    fn which_direction_am_i(&self) -> &'static str {
        match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        }
    }
}

impl Coordinates {
    fn show_coordinates(&self) {
        println!("x:{}, y:{}", self.x, self.y);
    }
    fn change_coordinates(&mut self, new_coordinates: Vec<i8>) {
        self.x = new_coordinates[0];
        self.y = new_coordinates[1];
    }
}

impl Movement {
    fn change_direction(&self, direction: Direction, steps: u8, mut old_coordinates: Vec<i8>) -> Vec<i8> {
        // Value 0 of the old coordinates is x, value 1 is y.
        match direction {
            Direction::Up => {
                old_coordinates[1] = old_coordinates[1] + steps as i8;
            }
            Direction::Down => {
                old_coordinates[1] = old_coordinates[1] - steps as i8;
            }
            Direction::Right => {
                old_coordinates[0] = old_coordinates[0] + steps as i8;
            }
            Direction::Left => {
                old_coordinates[0] = old_coordinates[0] - steps as i8;
            }
        }
        old_coordinates
    }
}

fn main() {
    // Example Movement
    let mut my_coordinates = Coordinates { x: 0, y: 0 };
    let my_direction = Direction::Up;
    let my_movement = Movement {
        direction: my_direction.clone(),
        steps: 5,
    };
    let new_coordinates = my_movement.change_direction(my_direction, my_movement.steps, vec![my_coordinates.x as i8, my_coordinates.y as i8]);
    println!("{}", new_coordinates[0]);
    my_coordinates.change_coordinates(new_coordinates);
    my_coordinates.show_coordinates();
}
