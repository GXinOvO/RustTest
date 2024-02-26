/*
--TODO: ToString 
    要把任何类型转换成String，只需要实现那个类型的ToString train。然而不要直接这么做，
  应该实现fmt::Display trait，它会自动提供ToString，并且还可以用来打印类型。
*/
use std::fmt;
use std::string::ToString;

struct Circle 
{
    radius: i32
}

// impl fmt::Display for Circle
// {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
//     {
//         write!(f, "Circle of radius {}", self.radius)
//     }
// }

impl ToString for Circle
{
    fn to_string(&self) -> String 
    {
        format!("Circle of radius {:?}", self.radius)
    }
}

/*
--TODO: 解析字符串
    我们经常需要把字符串转成数字。完成这项工作的标准手段是用parse函数。我们得提供要转换到得
  类型，这可以通过不使用类型推断，或者用"涡轮鱼"语法(turbo fish, <>)实现。
    只要对目标类型实现了FromStr trait，就可以用parse把字符串转换成目标类型。标准库中已经给
  无数种类型实现了FromStr。如果要转换到用户定义类型，只要手动实现FromStr就行。
*/

fn main()
{
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}