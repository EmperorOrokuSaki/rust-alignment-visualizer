// Define a custom enum
enum Direction {
    North,
    South,
    East,
    West,
}

// Define a trait
trait Movable {
    fn move_forward(&self);
    fn move_backward(&self);
}

// Implement the trait for the Direction enum
impl Movable for Direction {
    fn move_forward(&self) {
        match self {
            Direction::North => println!("Moving forward - North"),
            Direction::South => println!("Moving forward - South"),
            Direction::East => println!("Moving forward - East"),
            Direction::West => println!("Moving forward - West"),
        }
    }

    fn move_backward(&self) {
        match self {
            Direction::North => println!("Moving backward - North"),
            Direction::South => println!("Moving backward - South"),
            Direction::East => println!("Moving backward - East"),
            Direction::West => println!("Moving backward - West"),
        }
    }
}

// Define a custom struct
#[repr(C)]
struct Car {
    model: String,
    year: u32,
    direction: Direction,
}

// Implement methods for the Car struct
impl Car {
    fn new(model: &str, year: u32, direction: Direction) -> Self {
        Car {
            model: model.to_string(),
            year,
            direction,
        }
    }

    fn display_info(&self) {
        println!("Model: {}", self.model);
        println!("Year: {}", self.year);
        println!("Direction: {:?}", self.direction);
    }
}

fn main() {
    // Create a car instance
    let car = Car::new("Toyota", 2022, Direction::North);

    // Display car information
    car.display_info();

    // Move the car forward and backward
    car.direction.move_forward();
    car.direction.move_backward();
}
