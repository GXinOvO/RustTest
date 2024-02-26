#![allow(dead_code)]

enum Number
{
    Zero,
    One,
    Two,
}

enum Color 
{
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main()
{
    // -> as 最常用于将原始类型转换为其他原始类型，但他还有其他用途，包括将指针转换为地址、地址转换为指针以及将指针转换为其他指针
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}