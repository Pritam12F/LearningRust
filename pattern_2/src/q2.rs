pub fn q2(){
    let mut sen: String = String::from("Hello");
    let r: &mut String = &mut sen;

    match r{
        value => {
            value.push_str(" World!");
            println!("Done!");
        },
    }
}