struct User {
    active: bool,
    username: String, //Don't use references in structs cause we want the struct to retain the onwership while the struct is up
    email: String,
    sign_in_count: u32
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct None; // Unit-style struct

fn main() {
    // Multable cannot be individual for each field
    let mut u = User {
        active: true,
        username: String::from("misterx"),
        email: String::from("mister@x.com"),
        sign_in_count: 1,
    };

    let u2 = user_initializer(u.username, u.email);

    // This will MOVE a struct to another struct this way 
    let inactive_user = User{
        active: false,
        ..u2
    };

    let black = Color(0,0,0);
    let nil = None;
}

fn user_initializer(username: String, email: String) -> User {
    // if the name of the variables is the same as the attribute, you can this method to initialize the field
    return User {
        active: true,
        username,
        email,
        sign_in_count: 2,
    }
}
