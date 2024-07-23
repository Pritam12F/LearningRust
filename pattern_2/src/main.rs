mod q1;
mod q2;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point {x: 1, y: 0};

    match point {
        Point{x, y: 0} => println!("Point is on x-axis"),
        Point{x: 0, y} => println!("Point is on y-axis"),
        Point{x, y: y @ (1 | 2 | 3)} => println!("Y axis is {}", y),
        _ => println!("Every other case"),
    }

    q1::q1();
    q2::q2();
}
