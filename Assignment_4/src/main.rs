enum Shape {Triangle, Square, Pentagon, Octagon}

impl Shape {
    fn corners(&self) -> i8 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Octagon => 8,
        }
    }
}

fn main() {
    let triangle = Shape::Triangle;
    let square = Shape::Square;
    let pentagon = Shape::Pentagon;
    let octagon = Shape::Octagon;

    println!("Triangle has {} corners", triangle.corners());
    println!("Square has {} corners", square.corners());
    println!("Pentagon has {} corners", pentagon.corners());
    println!("Octagon has {} corners", octagon.corners());
}