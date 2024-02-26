/*
--TODO:
    From trait允许一种类型定义"怎么根据另一种类型生成自己",因此他提供了一种类型转换的简单机制。
  在标准库中由无数From的实现，规定原生类型及其他常见类型的转换功能
    比如，可以很容易地把str转换成String
    let my_str = "hello";
    let mt_string = String::from(my_str);
*/
use std::convert::From;
#[derive(Debug)]
struct Number
{
    value: i32,
}
impl From<i32> for Number
{
    fn from(item: i32) -> Self
    {
        Number { value:item }
    }
}

/*
--TODO: 
    Into trait就是把From trait倒过来而已。也就是说，如果你为你的类型实现了From，那么同时
  免费获得了Into。
    使用Into trait通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。不过考虑到我
  们免费获得了Into，这点代价不值一提。
*/

fn main()
{
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let nums: Number = int.into();
    println!("My number is {:?}", nums);
}
