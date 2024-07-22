mod q1;
mod q2;

fn main() {
    let num = 0;

    if num > 0 {
        println!("Number is positive");
    }else if num < 0 {
        println!("Number is negative");
    }else {
        println!("Number is 0");
    }

    q1::checkNum();
    q2::main2();
}
