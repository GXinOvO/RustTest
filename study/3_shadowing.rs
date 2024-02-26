fn main()
{
    /*
        重影(shadowing)的概念与其他面向对象语言里的"重写"(Override)或"重载"(Overload)是不一样的，
      重影就是"重新绑定"
        重影就是指变量的名称可以被重新使用的机制

        重影与可变变量的赋值不是一个概念，重影是指用同一个名字重新代表另一个变量实体，其类型、可变属
      性和值都可以变化。但可变变量赋值仅能发生值得变化。
     */
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let mut s = "123";
    s = s.len();    // error
    println!("The len of s is: {}", s);
}