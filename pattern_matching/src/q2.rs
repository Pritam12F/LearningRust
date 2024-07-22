enum Message {
    Quit,
    Move{x: i32, y: i32},
    Word(String),
    Color(i32, i32, i32),
}

pub fn patternMatching() {
    let msgs = [
        Message::Quit,
        Message::Move{x: 1, y: 2},
        Message::Color(1, 2, 3)
    ];

    for msg in msgs{
        show_messaage(msg);
    }

    println!("Success!");
}

fn show_messaage(msg: Message){
    match msg {
        Message::Move{x: a, y: b} => {
            assert_eq!(a, 1);
            assert_eq!(b, 2);
        },
        Message::Color(_, g, b) => {
            assert_eq!(g, 2);
            assert_eq!(b, 3);
        },
        _ => println!("No data in this variant")
    }
}