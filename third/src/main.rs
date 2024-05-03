// Borrowing and references

fn main() {
    let mut str: String = String::from("hi there");
    let s2 = &mut str;
    let s3 = change_sentence(&mut str);
    // println!("{}", s2);
    println!("{}", s3);
}

fn change_sentence(s: &mut String) ->String {
    s.push_str(" world");
    return s.to_string();
}