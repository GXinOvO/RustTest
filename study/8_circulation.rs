fn main()
{
    let mut number = 1;
    while number != 4
    {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");

    /* --TODO: for */
    let a = [10, 20, 30, 40, 50];
    for i in a.iter()
    {
        println!("值为: {}", i);
    }
    for i in 0..5 
    {
        println!("a[{}] = {}", i, a[i]);
    }


    /* --TODO: loop 
        当某个循环无法在开头和结尾判断是否继续进行循环，必须在循环体中间
      某处控制循环的进行。如果遇到这种情况，我们经常会在一个while(true)循环
      体里实现中途退出循环的操作。而Rust语言有原生的无限循环结构 -- loop
    */
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop 
    {
        let ch = s[i];
        if ch == 'O'
        {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    };

    /* loop循环可以通过break关键字类似于return一样使整个循环退出并给予外部一个返回值。*/
    let mut j = 0;
    let location = loop {
        let ch = s[j];
        if ch == 'O'
        {
            break j;
        }
        j += 1;
    };
    println!("\'O\'的索引为{}", location);

}