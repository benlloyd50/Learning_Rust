fn main() {
    // let user1 = User {
    //     email: String::from("emailme@email.com"),
    //     username: String::from("benboyd123"),
    //     active: true,
    //     sign_in_count: 20,
    // };
    let user1 = build_user("mycoolemail@coolio.com", "benhomer29");

    let user2 = User {
        email: String::from("sofie@yadayda.com"),
        ..user1
    };

    println!("Good morning {}", user1.email);

}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: email.to_string(),
        username: username.to_string(),
        active: true,
        sign_in_count: 69,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}