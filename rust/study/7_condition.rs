/*
    Rust中条件表达式与 C 会有一些区别: 条件表达式不需要用 () 包括(注意，不需要
  不是不允许); 但是Rust中的if不存在单语句不用加{}的规则，不允许使用一个语句代替
  一个块。尽管如此，Rust还是支持传统else-if语法的

*/

fn main()
{
    /* --TODO: if */
    let number = 3;
    if number < 5
    {
        println!("条件为true");
    }
    else
    {
        println!("条件为false");
    }

    /* --TODO: else if */
    let a = 12;
    let b;
    if a > 0 
    {
        b = 1;
    }
    else if a < 0
    {
        b = -1;
    }
    else 
    {
        b = 0;
    }
    println!("b is {}", b);

    /* --TODO: only bool */
    /*
        /* 报错: expected `bool`, found integerrustc(E0308) */
        let number = 3;
        if number 
        {
            println!("YES");
        }
     */



    
    /* --TODO: only condition 
        
        if <condition> { block 1 } else { block 2 }
        其中{ block 1 } 和 { block 2 }可以是函数体表达式
    */
    let c = 3;
    let size = if c > 0 { 1 } else { -1 };
    println!("size 为 {}", size);
}