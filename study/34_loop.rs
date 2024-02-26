/*
--TODO: loop
    Rust提供了loop关键字来表示一个无限循环
    可以使用break语句在任何时候退出一个循环，还可以使用continue跳过循环体的
  剩余部分并开始下一轮循环。
*/

// fn main()
// {
//     let mut count = 0u32;

//     println!("Let's count until infinity!");

//     loop
//     {
//         count += 1;
//         if count == 3
//         {
//             println!("three");
//             continue;
//         }
//         println!("{}", count);
//         if count == 5
//         {
//             println!("Ok, that's enough");
//             break;
//         }
//     }
// }

#![allow(unreachable_code)]

fn main()
{
    'outer: loop
    {
        println!("Entered the outer loop");

        'inner: loop
        {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}
