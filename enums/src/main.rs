enum Direction {
    north,
    south,
    west,
    east,
}

enum Shape {
    Circle(f32),
    Square(f32),
    rectangle(f32, f32),
}

fn main() {
    /*
    Enums allows you to define a type by enumerating its possible variants
    */

    let my_direction = Direction::south;
    current_direction(my_direction);

    let circle = Shape::Circle(5.0);
    let rectangle = Shape::rectangle(5.5, 2.5);
    let square = Shape::Square(3.5);

    let area=calculate_area(square);
    println!("Area: {}",area)
}
fn current_direction(direction: Direction) {
    //Login Here
}

fn calculate_area(shape: Shape) -> f32 {
    let area = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) => {
            println!("Square");
            side * side
        }
        Shape::rectangle(width, hiegth) => width * hiegth,
    };
    return area;
}
