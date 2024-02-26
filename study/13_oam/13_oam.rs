/*
    --TODO: 箱(Crate)
    "箱"是二进制程序文件或者库文件，存在于"包"中。
    "箱"是树状结构的，它的树根是编译器开始运行时编译的源文件所编译的程序。  
    注意："二进制程序文件"不一定是"二进制可执行文件"，只能确定是是包含目标机器语言的
  文件，文件格式随编译环境的不同而不同。


    --TODO: 包(Package)
      当我们使用 Cargo 执行 new 命令创建 Rust 工程时，工程目录下会建立一个 Cargo.toml 文件。
    工程的实质就是一个包，包必须由一个 Cargo.toml 文件来管理，该文件描述了包的基本信息以及依赖
    项。一个包最多包含一个库"箱"，可以包含任意数量的二进制"箱"，但是至少包含一个"箱"（不管是库还
    是二进制"箱"）。
      当使用 cargo new 命令创建完包之后，src 目录下会生成一个 main.rs 源文件，Cargo 默认这个
    文件为二进制箱的根，编译之后的二进制箱将与包名相同。


    --TODO: 模块(Module)
      对于一个软件工程来说，我们往往按照所使用的编程语言的组织规范来进行组织，组织模块的主要结构往
    往是树。Java 组织功能模块的主要单位是类，而 JavaScript 组织模块的主要方式是 function。
      这些先进的语言的组织单位可以层层包含，就像文件系统的目录结构一样。Rust 中的组织单位是模块（Module）。


    mod nation 
    {
        mod government
        {
            fn govern() {}
        }
        mod congress
        {
            fn legislate() {}
        }
        mod court
        {
            fn judicial() {}
        }
    }
      这是一段描述法治国家的程序：国家（nation）包括政府（government）、议会（congress）和法院（court），
    分别有行政、立法和司法的功能。我们可以把它转换成树状结构：
    nation
    ├── government
    │ └── govern
    ├── congress
    │ └── legislate
    └── court
    └── judicial
      在文件系统中，目录结构往往以斜杠在路径字符串中表示对象的位置，Rust 中的路径分隔符是 :: 。
      路径分为绝对路径和相对路径。绝对路径从 crate 关键字开始描述。相对路径从 self 或 super 关
    键字或一个标识符开始描述。例如：
    crate::nation::government::govern();
      是描述govern函数的绝对路径，相对路径可以表示为:
      nation::government::govern();

      尝试在一个源程序里定义类似的模块结构并在主函数中使用路径。
      如果这样做，一定会发现它不正确的地方：government 模块和其中的函数都是私有（private）的，你不
    被允许访问它们。
 */

 /*
    --TODO: 访问权限
        Rust中有两种简单的访问权: 公共(public)和私有(private).
        默认情况下，如果不加修饰符，模块中的成员访问权将是私有的。
        如果想使用公共权限，需要使用pub关键字。
        对于私有的模块，只有在与其平级的位置或下级的位置才能访问，不能从其外部访问。
  */

mod nation
{
    pub mod government
    {
        pub fn govern() {}
    }
    mod congress 
    {
        pub fn legislate() {}
    }
    mod court
    {
        fn judicial()
        {
            super::congress::legislate();
        }
    }
}

mod back_of_house
{
    pub struct Breakfast
    {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast 
    {
        pub fn summer(toast: &str) -> Breakfast
        {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant()
{
    let mut meal = back_of_house::Breakfast::summer("Rys");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

/*
  枚举类枚举项可以内含字段，但不具备类似的性质:
*/

mod SomeModule
{
  pub enum Person
  {
    King { name: String },
    Queen
  }
}


fn main()
{
  nation::government::govern();
  eat_at_restaurant();
  let person = SomeModule::Person::King{
    name: String::from("Blue")
  };
  match person
  {
    SomeModule::Person::King {name} =>
    {
      println!("{}", name);
    }
    _ => {}
  }

  /*
    --TODO: 难以发现的模块
      使用过Java的开发者在编程时往往非常讨厌最外层的class块 -- 他的名字与文件名一摸一
    样，因为他就表示文件容器，尽管他很繁琐但我们不得不写一遍来强调"这个类是文件所包含的类"
      不过这样有一些好处: 起码它让开发者明明白白的意识到了类包装的存在，而且可以明确的描述
    类的继承关系。
      在Rust中，魔魁啊就像是Java中的类包装，但是文件一开头就可以写一个主函数，这该如何解释呢？
      每一个Rust文件的内容都是一个"难以发现"的模块。  
    
      见 main.rs 和 second.module.rs文件
    */

}