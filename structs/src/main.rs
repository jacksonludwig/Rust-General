fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let mut user2 = build_user(String::from("someemail@intenet.com"),
                               String::from("aNewuser"));

    user2 = User {
        email: String::from("newEmail"), // update email
        username: String::from("newName"), // update username
        ..user1 // copies rest of fields from user1 over user 2
    };

    struct Color(i32, i32, i32); // tuple struct
    let black = Color(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // implicit assignment
        username, // implicit assignment
        active: true,
        sign_in_count: 1,
    }
}