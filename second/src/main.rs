fn main() {
    
    // Ownership Example

    let num1: String = String::from("Rihanna");
    let _num2: String = num1.clone();

    println!("{}", num1);
}
