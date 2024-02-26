mod second;
use second::ClassName;

fn main()
{
    let object = ClassName::new(1024);
    object.public_method();

    /*
        --TODO: 继承
            几乎其他的面向对象的编程语言都可以实现"继承"，并用"extand"词语来描述这个动作。
            继承是多态(Polymorphism)思想的实现，多态指的是编程语言可以处理多种类型数据的代码。
            在Rust中，通过特性(trait)实现多态。但是特性无法实现属性的继承，只能实现类似于"接口"
          的功能，所以想继承一个类的方法最好在"子类"中定义"父类"的实例。
     */
}