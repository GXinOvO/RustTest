/*
    --TODO: Rust面向对象
        面向对象的编程语言实现了数据的封装与继承并能基于数据调用方法。
        Rust不是面向对象的编程语言，但这些功能都得以实现。
*/

/*
    --TODO: 封装
        封装就是对外显示的策略，在Rust中可以通过模块的机制来实现最外层的封装，并且每个
      Rust文件都可以看作一个模块，模块内的元素可以通过pub关键字对外明示。
        "类"往往是面向对象的编程语言中常用到的概念。"类"封装的是数据，是对同一类数据实体
      以及其处理方法的抽象。在Rust中，我们可以使用结构体或枚举类来实现类的功能
*/
pub struct ClassName
{
    field: i32,
}

impl ClassName 
{
    pub fn new(value: i32) -> ClassName
    {
        ClassName {
            field: value
        }
    }

    pub fn public_method(&self)
    {
        println!("from public method");
        self.private_method();
    }

    fn private_method(&self)
    {
        println!("from private method");
    }
}
