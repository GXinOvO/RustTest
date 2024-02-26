/*
--TODO: super和self
    可以在路径中使用super(父级)和self(自身)关键字，从而在访问项时消除歧义，以及防止不必要的
  路径硬编码
*/

fn function()
{
    println!("called `function()`");
}

mod cool
{
    pub fn function()
    {
        println!("called `cool::function()`");
    }
}

mod my
{
    fn function()
    {
        println!("called `my::function()`");
    }

    mod cool
    {
        pub fn function()
        {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call()
    {
        println!("called `my::indirect_call()`, that\n> ");

        self::function();
        function();

        // -> 我们也可以使用`self`来访问`my`内部的另一个模块
        self::cool::function();

        // -> `super`关键字表示父作用域(在`my`模块外面)
        super::function();

        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main()
{
    my::indirect_call();
}