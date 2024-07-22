pub fn checkNum(){
    let num = 10;

    let big_num = if num > -10 && num < 10 {
        println!("{} is a big number", num);

        num / 10
    } else {
        println!("{} is a big number", num);

        num / 2
    };

    println!("{} -> {}", num, big_num);
}