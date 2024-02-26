/*
    --TODO: kind方法
    Rust似乎没有像try块一样可以令任何位置发生的同类异常都直接得到相同的解决的语法，但这样并不意味着Rust实现不了: 我们完全
  可以把try块在独立的函数中实现，将所有的异常都传递出去解决。实际上这才是一个分化良好的程序应当遵循的编程方法: 应该注重
  独立功能的完整性。
    但是这样需要判断Result的Err类型，获取Err类型的函数是kind()
*/
use std::io;
use std::io::Read;
use std::fs::File;

fn read_text_from_file(path: &str) -> Result<String, io::Error>
{
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main()
{
    let str_file = read_text_from_file("hello.txt");
    match str_file
    {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                },
                _ => {
                    println!("Cannot read the file");
                }
            }
        }
    }
}