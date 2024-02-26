/*
  --TODO: error
    Rust有一套独特的处理异常情况的机制，他并不像其他语言中的try机制那样简单。
    首先，程序中一般会出现两种错误: 可恢复错误和不可恢复错误。
    可恢复错误的典型案例是文件访问错误，如果访问一个文件失败，有可能是因为它正在
  被占用，是正常的，我们可以通过等待来解决。
    但还有一种错误是由编程中无法解决的逻辑错误导致的，例如访问数组末尾以外的位置。
    大多数编程语言不区分这两种错误，并用Exception(异常)类来表示错误，在Rust种没有Exception
    对于可恢复错误用Result<T, E>类来处理，对于不可恢复错误使用panic!宏来处理
 */

use std::fs::File;

fn h(i: i32) -> Result<i32, bool> 
{
    if i >= 0 {Ok(i)}
    else { Err(false) }
}

fn g(i: i32) -> Result<i32, bool>
{
    let t = h(i);
    return match t {
        Ok(i) => Ok(i),
        Err(b) => Err(b)
    };
}

fn w(i: i32) -> Result<i32, bool> {
    let t = h(i)?;
    Ok(t);  // 因为确定t不是Err,t在这里已经是i32类型
}

 fn main()
 {
     /* 
         程序是无法运行到println!，而是在panic!宏被调用时停止了运行
         不可恢复的错误一定会导致程序收到致命的打击而终止运行
     thread 'main' panicked at 'error occured', src\main.rs:3:5
     note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
         第一行输出了panic!宏调用的位置以及其输出的错误信息
         第二行是一句提示，翻译成中文就是 "通过'RUST_BACKTRACE=1'环境变量运行以显示回溯"
 
         RUST_BACKTRACE=1 cargo run
      */


    /* 
     panic!("error occured");
     println!("Hello, Rust");
    */
    
    /*
        回溯是不可恢复错误的另一种方式，他会展开运行的栈并输出所有的信息，然后程序依然会退出。
      */


    /*
    --TODO: 可恢复的错误
        此概念十分类似Java编程语言种的异常。实际上在C语言中我们就常常将函数返回值设置成整数
      来表达函数遇到的错误，在Rust中通过Result<T, E>枚举类作返回值来进行异常表达:
        enum Result<T, E> 
        {
            Ok(T),
            Err(E),
        }

        在Rust标准库中可能产生异常的函数的返回值都是Result类型的。
     */
    let f = File::open("hello.txt");
    /*
        普通版本
    match f
    {
        Ok(file) => {
            println!("File Opened successfully.");
        },
        Err(err) => {
            println!("Failed to open the file.")
        }
    }
    */

    /* --TODO: 优化 */
    if let Ok(file) = f {
        println!("File Opened successfully.");
    } else {
        println!("Failed to open the file.");
    }

    /*
        --TODO: 如果想使一个可恢复错误按不可恢复错误处理，Result类提供了两个办法: unwrap()和expect(message: &str):
    let f1 = File::open("hello.txt").unwrap();
    let f2 = File::open("hello.txt").expect("Failed to open.");
        相当于在Result为Err时调用panic!宏。两者的区别在于expect能够向panic!宏发送一段指定的错误信息。
     */
    let r = h(10000);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }

    let r1 = g(10000);
    if let Ok(v) = r1 {
        println!("Ok: g(10000) = {}", v);
    } else {
        println!("Err");
    }


    /*
        ? 符的实际作用是将Result类非异常的值直接取出,如果有异常就将异常Result返回出去。所以, ? 符仅用于返回值类型为Result<T, E>的函数，其中E类型
      必须和 ? 所处理的Result的E类型一致。
     */
}