/*
--TODO: 作为输入参数
    虽然Rust无需类型说明就能在大多数时候完成变量捕获，但在编写函数时，这种模糊写法是不允许的。
  当以闭包作为输入参数时，必须指出闭包的完整类型，他是通过使用一下trait中的一种来指定的。其受限制程度按以下顺序递减:
    · Fn: 表示捕获方式为通过引用(&T)的闭包
    · FnMut: 表示捕获方式为通过可变引用(&mut T)的闭包
    · FnOnce: 表示捕获方式为通过值(T)的闭包
译注:
    顺序之所以是这样，是因为&T知识获取了不可变的引用，&mut T则可以改变变量，T则是拿到了变量的所有权而非借用。

    对闭包所要捕获的每个变量，编译器都将以限制最少的方式来捕获
译注:
    这句可能不对，事实上是在满足使用需求的前提下尽量以限制最多的方式捕获。
*/
fn apply<F>(f: F) where
    F: FnOnce() {
        f();
    }

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
        f(3)
    }

fn main()
{
    use std::mem;

    let greeting = "hello";

    // -> to_owned 从借用的数据创建有所有权的数据
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.",greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.",farewell);
        println!("Now I can sleep. zzzzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double =  |x| 2 *  x;
    println!("3 doubled: {}", apply_to_3(double));
}