/*
--TODO: 绑定
    在match种，若间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它。match
  提供了@符号来绑定变量到名称:
*/
fn age() -> u32
{
    15
}

// -> 你也可以使用绑定来解构enum变体，例如Option:
fn some_number() -> Option<u32>
{
    Some(42)
}

fn main()
{
    println!("Tell me what type of person you are");

    match age()
    {
        0              => println!("I haven't celebrated my first birthday yet"),
        n @ 1   ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13  ..= 19 => println!("I'm a teen of age {:?}", n),
        n              => println!("I'm an old person of age {:?}", n),
    }

    match some_number()
    {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n)      => println!("Not interesting... {}", n),
        _            => (),
    }
}