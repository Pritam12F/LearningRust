fn main() {
    let is_male: bool = true;
    let is_legal: bool = true;

    if is_male{
        println!("Person is male");
    }
    else{
        println!("Person is female");
    }
    if is_legal{
        println!("Person is Legal");
    }
    else {
        println!("Person is not legal age");
    }
    if is_legal && is_male{
        println!("Person is a legal male");
    }

    let name1: &str = "pritam";

    let char1 = name1.chars().nth(0);

    match char1{
        Some(c) => println!("{}", c),
        None => println!("Value does not exist")
    }

    let name: String = String::from("my name is pritam");
    let first_word = get_first_word(name);

    println!("{}", first_word);
}

fn get_first_word(name: String) -> String{
    let mut first_w: String = String::new();
    for (_i,c) in name.char_indices(){
        if c==' '{
            break;
        }
        first_w.push_str(&c.to_string());
    }

    return first_w;
}