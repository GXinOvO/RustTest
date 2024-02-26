/*
--TODO: 类型匿名
    当闭包被定义，编译器会隐式地创建一个匿名类型的结构体，用以储存闭包捕获的变量，同时为这个未知
  类型的结构体实现函数功能，通过Fn、FnMut或FnOnce三种trait中的一种。
    若使用闭包作为函数参数，由于这个结构体的类型未知，任何的用法都要求是泛型的。然而，使用未限定
  类型的参数<T>过于不明确，并且是不允许的。事实上，指明为该结构体实现的是Fn、FnMut、或FnOnce中的
  哪种trait，对于约束该结构体的类型而言就已经足够了。
*/

// -> `F`必须为一个没有输入参数和返回值的闭包实现`Fn`，这和对`print`的要求恰好一样
fn apply<F>(f: F) where
    F: Fn() {
        f();
    }

/*
--TODO: 输入函数
    既然闭包可以作为参数，函数当然也可以。如果你声明一个接受闭包作为参数的函数，那么任何
  满足该闭包的trait约束的函数都可以作为其参数。
*/
fn call_me<F: Fn()>(f: F)
{
    f()
}

fn function()
{
    println!("I'm a function!");
}

/*
--TODO: 作为输出参数
    闭包作为输入参数是可能的，所以返回闭包作为输出参数也应该是可能的。然而返回闭包类型会有问题，
  因为目前Rust只支持返回具体(非泛型)的类型。按照定义，匿名的闭包的类型是未知的，所以只有使用
  impl Trait才能返回一个闭包。

返回闭包的有效特征是:
    · Fn
    · FnMut
    · FnOnce

    除此之外，还必须使用move关键字，它表明所有的捕获都是通过值进行的。这是必须的，因为在函数退出时，
  任何通过引用的捕获都被丢弃，在闭包中留下无效的引用。
*/
fn create_fn() -> impl Fn()
{
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut()
{
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce()
{
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main()
{
    let x = 7;
    let print = || println!("{}", x);

    apply(print);

    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}