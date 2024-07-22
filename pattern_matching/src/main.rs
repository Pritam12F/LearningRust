mod q1;
mod q2;

fn main() {
    enum Direction {
        North,
        South,
        East,
        West
    }

    let dir = Direction::North;

    match dir {
        Direction::North | Direction::South => println!("North or South"),
        Direction::East | Direction::West => println!("East or West")
    }

    q1::main2();
    q2::patternMatching();
}
