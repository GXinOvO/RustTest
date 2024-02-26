/*
--TODO: 作用域和遮蔽
    变量绑定有一个作用域(scope)，他被限定只在一个代码块(block)中生存(live)。代码块是一个被{}包围的语句集合。
  另外也允许变量遮蔽。
*/
fn main()
{
    let long_lived_bingding = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        let long_lived_bingding = 5_f32;
        println!("inner long: {}", long_lived_bingding);
    }

    // println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_bingding);

    // -> 变量遮蔽
    let long_lived_bingding = 'a';
    println!("outer long: {}", long_lived_bingding);

    /*
    --TODO: 冻结
        当数据被相同的名称不变地绑定时，他还会冻结(freeze)。在不可变绑定超出作用域之前，无法修改已冻结的数据
     */
    
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;

        _mutable_integer = 50;

    }
    _mutable_integer = 3;
}