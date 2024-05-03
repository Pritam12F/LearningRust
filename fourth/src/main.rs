// Structs
// Implement Structs

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn find(&self) -> u64 {
        self.sign_in_count*3
    }
}

fn main() {
    // let user1 = User {
    //     active: true,
    //     username: String::from("pritam2"),
    //     email: String::from("pritam.das.santu@gmail.com"),
    //     sign_in_count: 1,
    // };

    // println!("{:?}", user1.username);

    let user2 = User {
        active: true,
        username: String::from("pritam"),
        email: String::from("pritam@gmail.com"),
        sign_in_count: 2,
    };

    println!("{:?}", user2.find());
}
