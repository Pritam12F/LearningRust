// Enums and pattern matching

enum Shapes {
    Square(f64),
    Rectangle(f64, f64),
    Circle(f64),
    Triangle(f64, f64)
}

fn find_area(shapes: Shapes) -> f64 {
    match shapes {
        Shapes::Square(side) => side * side,
        Shapes::Rectangle(len, brd) => 2.0 * (len + brd),
        Shapes::Circle(radius) => 3.14 * radius * radius,
        Shapes::Triangle(base, height) => 0.5 * base * height,
    }
}
fn main() {
    let area_square = find_area(Shapes::Square(5.25));
    let area_react = find_area(Shapes::Rectangle(3.3, 3.5));
    let area_circle = find_area(Shapes::Circle(2.3));
    let area_triangle = find_area(Shapes::Triangle(2.2, 4.5));

    println!("Area of square is {}, Area of rectangle is {}, Area of circle is {}, Area of triangle is {}", area_square, area_react, area_circle, area_triangle);
}
