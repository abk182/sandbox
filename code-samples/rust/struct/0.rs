fn main() {
    try_struct()
}

enum Status {
    Alive,
    Dead,
}

struct Birth(i32, i32, i32);

struct User {
    status: Status,
    username: String,
    email: String,
    sign_in_count: u64,
    birth: Birth,
}

fn try_struct() {
    struct EmptyStruct;
    let emptyStruct = EmptyStruct;

    let mut user0 = User {
        status: Status::Dead,
        username: String::from("Username"),
        email: String::from("email@mail.com"),
        sign_in_count: 0,
        birth: Birth(16, 12, 92),
    };

    let mut user1 = User {
        status: Status::Alive,
        sign_in_count: user0.sign_in_count + 1,
        ..user0
    };

    user0.username = String::from("*");
    user1.email = String::from("awesome@mail.com");

    println!("{0}", user0.username);
    // println!("{0}", user0.birth.0); wont work, borrowing
    // println!("{0}", user0.email); wont work, borrowing
    println!(
        "{0}, {1}.{2}.{3}",
        user1.username, user1.birth.0, user1.birth.1, user1.birth.2
    );
}
