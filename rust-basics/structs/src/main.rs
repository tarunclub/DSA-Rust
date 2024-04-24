#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("tarun"),
        email: String::from("tarun@gmail.com"),
        sign_in_count: 32,
        active: true,
    };

    println!("{:#?}", user1);

    user1.username = String::from("tarunkm");

    println!("{:#?}", user1.username);

    let user2 = build_user(String::from("hello@gmail.com"), String::from("Hello"));

    println!("{:#?}", user2);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {:#?}", rect.area());
    println!("{:#?}", rect);

    let rect3 = Rectangle::square(10);

    println!("{:#?}", rect3.area())
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
