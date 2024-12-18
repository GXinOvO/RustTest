/*
--TODO: 模块
    Rust提供了一套强大的模块(module)系统，可以将代码按层次分成多个逻辑单元(模块),
  并管理这些模块之间得可见性(公有(public)或私有(private))

    模块是项(item)的集合，项可以是: 函数，结构体，trait，impl块，甚至其他模块


--TODO: 可见性
    默认情况下，模块中的项拥有私有的可见性(private visibility)，不过可以加上`pub`修饰语来
  重载这一行为。模块中只有公有的(public)项可以从模块外的作用域访问。
*/

mod my_mod
{
    // -> 模块中的项默认具有私有的可见性
    fn private_function()
    {
        println!("called `my_mod::private_function()`");
    }

    // -> 使用`pub`修饰语来改变默认可见性
    pub fn function()
    {
        println!("called `my_mod::function()`");
    }

    // -> 在同一模块中，项可以访问其他项，即使他是私有的
    pub fn indirect_access()
    {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // -> 模块也可以嵌套
    pub mod nested
    {
        pub fn function()
        {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function()
        {
            println!("called `my_mod::nested::private_function()`");
        }

        // -> 使用`pub(in path)`语法定义的函数只在给定的路径中可见。
        // -> `path`必须是父模块(parent module)或祖先模块(ancestor module)
        pub(in crate::my_mod) fn public_function_in_my_mod()
        {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // -> 使用`pub(self)`语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested()
        {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // -> 使用`pub(super)`语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod()
        {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod()
    {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)`使得函数只在当前crate中可见
    pub(crate) fn public_function_in_crate()
    {
        println!("called `my_mod::public_function_in_crate()");
    }

    // -> 嵌套模块的可见性遵循相同的规则
    mod private_nested
    {
        #[allow(dead_code)]
        pub fn function()
        {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function()
{
    println!("called `function()`");
}

fn main()
{
    // -> 模块机制消除了相同名字的项之间的歧义
    function();
    my_mod::function();

    // -> 公有项，包括嵌套模块内的，都可以在父模块外部访问
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // -> pub(crate) 项可以在同一个crate中的任何地方访问
    my_mod::public_function_in_crate();
}