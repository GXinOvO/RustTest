/*
    fn <函数名> (<参数>) <函数体>

    Rust不在乎在何处定义函数，只需在某个地方定义他们即可

    Rust中，函数定义可以嵌套
 */

fn main()
{
    println!("Hello, world!");
    another_function();
    another_function_num(5, 6);

    /*
        error
        let a = (let b = 2);
     */

    let x = 5;

    let y = {
        let a = 3;
        a + 1
    };

    println!("x 的值是: {}", x);
    println!("y 的值是: {}", y);

    fn five() -> i32
    {
        5
    }
    println!("five()的值为: {}", five());
}

fn another_function()
{
    println!("Hello, runoob!");
}

fn another_function_num(x: i32, y: i32)
{
    println!("x 的值为: {}", x);
    println!("y 的值为: {}", y);
}

/*
    Rust函数体由一系列可以以表达式(Expression)结尾的语句(Statement)组成。到目前为止, 我们仅见到了没有
  以表达式结尾的函数, 但已经将表达式用作语句的一部分
    语句是执行某些操作且没有返回值的步骤。
*/

/*
    Rust函数声明返回值类型的方式: 在参数声明之后用 -> 来声明函数返回值的类型(不是 :)
    在函数体中，随时都可以以 return 关键字结束函数运行并返回一个类型合适的值。
    但是Rust不支持自动返回值类型判断! 如果没有明确声明函数返回值的类型，函数将被认为是
  "纯过程",不允许产生返回值，return后面不能由返回值表达式。这样做的目的是为了让公开的
  函数能够形成可见的公报。

  注意:
    函数体表达式并不能等同于函数体，它不能使用return关键字
*/

fn add(a: i32, b: i32) -> i32
{
    return a + b;
}