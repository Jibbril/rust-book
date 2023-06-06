struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_account: u64
}

struct Color(i32,i32,i32);

fn main() {
    define_user();
    mut_user();

    let email = String::from("jibbril.b.n@gmail.com");
    let username = String::from("jibbril");
    construct_user(email, username);

    struct_update_syntax();
    tuple_structs();
}

fn define_user() {
    let user1 = User {
        email: String::from("jibbril.b.n@gmail.com"),
        username: String::from("jibbril"),
        active: true,
        sign_in_account: 1
    };

    println!("User email: {}",user1.email);
}

fn mut_user() {
    let mut user1 = User {
        email: String::from("jibbril.b.n@gmail.com"),
        username: String::from("jibbril"),
        active: true,
        sign_in_account: 1
    };
    user1.email = String::from("anotheremail@gmail.com");
    println!("User email: {}",user1.email);
}

fn construct_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_account: 1
    }
}

fn struct_update_syntax() {
    let email = String::from("jibbril.b.n@gmail.com");
    let username = String::from("Jibbril");
    let user1 = construct_user(email, username);
    let user2 = User {
        email: String::from("newemail@gmail.com"),
        ..user1 // Moves data from user1, user1 is no longer usable. 
    };
}

fn tuple_structs() {
    let red = Color(255,0,0);
    let green = Color(0,255,0);
    println!("Red: {}, Green: {}, Blue: {}",red.0, red.1, red.2);
}