pub fn main2(){
    let some = Some(10);

    // Pattern matching
    
    // match some {
    //     Some(des) => println!("Value is {}", des),
    //     _ => println!("Not some!"),
    // }

    // If let

    if let Some(des) = some {
        println!("Value is {}", des);
    }
}