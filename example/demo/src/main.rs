// struct User 
// {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
    
// }

// fn main() {
//     println!("Hello, world!");

//     let user1 = User {
//         email: String::from("acb@126.com"),
//         username: String::from("Nikky"),
//         active: true,
//         sign_in_count: 556,
//     };
// }
#[derive(Debug)]
struct Rectangle
{
    width: u32,
    length: u32,
}

impl Rectangle
{
    fn area(&self) -> u32 
    {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle 
    {
        Rectangle {
            width: size,
            length: size
        }
    }
}

fn main()
{
    let square = Rectangle::square(40);
    println!("square {:?}", square);
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };

    let rect3 = Rectangle {
        width: 35,
        length: 55,
    };


    
    println!("{}", rect1.area());
    println!("{:#?}", rect1);

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect2.can_hold(&rect3));
}

// fn area(rect: &Rectangle) -> u32 
// {
//     rect.width * rect.length
// }