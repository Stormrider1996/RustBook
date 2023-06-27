// defining a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// decalring a user
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}

// Changing the value in the email field of a User instance
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

// shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // instead of username: username
        email,    // instead of email: email
        sign_in_count: 1,
    }
}

// update syntax
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
} // instead of  let user2 = User {
  //     active: user1.active,
  //     username: user1.username,
  //     email: String::from("another@example.com"),
  //     sign_in_count: user1.sign_in_count,
  // };
  // Note that the struct update syntax uses = like an assignment; this is because it moves the data, just as we saw in the “Variables and Data Interacting with Move” section. In this example, we can no longer use user1 as a whole after creating user2 because the String in the username field of user1 was moved into user2. If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. Both active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
//Note that the black and origin values are different types because they’re instances of different tuple structs. Each struct you define is its own type, even though the fields within the struct might have the same types. For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.

//Unit-Like Structs Without Any Fields
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
