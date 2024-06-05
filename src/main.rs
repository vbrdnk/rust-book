fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    // struct update
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
