/*
--TODO: 闭包
    Rust中的闭包(closure),也叫做lambda表达式或者lambda，是一类能够捕获周围作用域中
  变量的函数。例如，一个可以捕获x变量的闭包如下:
    |val| val + x

    他们的语法和能力使他们在临时(on the fly)使用时相当方便。调用一个闭包和调用一个函数
  完全相同，不过调用闭包时，输入和返回类型两者都可以自动推导，而输入变量名必须指明。

    其他的特点包括:
        · 声明时使用||替代()将输入参数括起来
        · 函数体定界符({})对于单个表达式是可选的，其他情况必须加上。
        · 有能力捕获外部环境的变量
*/
fn main()
{
    // -> 函数实现
    fn function (i: i32) -> i32 { i + 1 }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred =  |i     |          i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one()); 
}