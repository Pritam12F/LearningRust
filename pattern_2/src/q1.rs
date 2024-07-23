enum Message {
    Hello { id: i32 },
}

pub fn q1(){
    let msg = Message::Hello{id: 5};

    match msg {
        Message::Hello{id: id @ 1..=3} => println!("Got a value between 1 and 3"),
        Message::Hello{id: newid @ 4..=9} => println!("Got a value between 4 and 9"),
        _ => println!("Either less than 0 or greater than 9")
    }
}