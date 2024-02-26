/*
--TODO: 发散函数
    发散函数(diverging function)绝不会返回。他们使用!标记，这是一个空类型
*/
fn foo() -> !
{
    panic!("This  call never returns.");
}

/*
    和所有其他类型相反，这个类型无法实例化，因为此类型可能具有的所有可能值得集合为空。
    注意，他与()类型不同，后者只有一个可能得值。
*/

// -> 下面函数中，虽然返回值中没有信息，但此函数会照常返回
fn some_fn()
{
    ()
}

// -> 下面这个函数相反，这个函数永远不会将控制内容返回给调用者
// #![feature(never_type)]


fn main()
{
    let a: () = some_fn();
    println!("This function returns and you can see this line.");


    // -> 2
    // let x: ! = panic!("This call never returns.");
    // println!("You will never see this line!");


    /*
        虽然这看起来像是一个抽象得概念，但实际上这非常有用且方便。这种类型得主要优点
      是它可以被转换为任何其他类型，从而可以在需要精确类型得地方使用，例如在match匹配
      分支:
    */
    fn sum_odd_numbers(up_to: u32) -> u32
    {
        let mut acc = 0;
        for i in 0..up_to 
        {
            let addition: u32 = match i % 2 == 1
            {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}