
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active:bool,
}

struct Color(i32, i32,i32);
struct Point(f32,f32,f32);

#[derive(Debug)]
struct Rectangle {
    width : u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }

    // Associated function.
    fn square(size: u32) -> Rectangle {
        return Rectangle{
            width: size,
            height: size,
        };
    }
} 


fn main() {

    let mut user1 = User {
        email: String::from("johnsmith@mail.com"),
        username: String::from("johnsmith"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("othermail@mail.com");

    let user2 = User {
        email: String::from("Rogerwilco@mail.com"),
        username: String::from("RogerWilco"),
        ..user1
    };
    let black = Color(0,0,0);
    let origin = Point(0.0, 0.0, 0.0);

    let rect1 = Rectangle { width: 30, height:50};
    println!("The area is {}", compute_area(&rect1));
    println!("The area is {}", rect1.area());
    println!("Rectangle {:?}", rect1);
    println!("Rectangle {:#?}", rect1);
    let rect2 = Rectangle{width:20, height:45};
    println!("The area of rect2 is {}", rect2.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("A square: {:#?}", Rectangle::square(10));

}

fn new_user(email: String, username: String) -> User {
    return User {
        email,
        username,  // Shorthand when field and variables are the same.
        active: true,
        sign_in_count: 1,
    }
}

fn compute_area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}