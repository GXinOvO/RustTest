/*
    --TODO: 特性:
        特性(trait)概念接近于Java中的接口(Interface)，但两者不完全相同。特性与接口相同的地方在于
      他们都是一种行为规范，可以用于标识哪些类有哪些方法。
        特性在Rust中用trait标识:
        trait Descriptive {
            fn describe(&self) -> String;
        }

        Descriptive规定了实现这必需有desctribe(&self) -> String方法。

    格式是:
        impl <特性名> for <所实现的类型名>
    
    Rust同一个类可以实现多个特性，每个impl块只能实现一个

*/


/*
    --TODO: 默认特性
        这是特性与接口的不同点，接口只能规范方法而不能定义方法，但特性可以定义方法作为默认方法，因为
      是"默认"，所以对象既可以重新定义方法，也可以不重新定义方法使用默认的方法:
*/
trait Descriptive {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}

struct Person {
    name: String,
    age: u8
}

impl Descriptive for Person {
    // fn describe(&self) -> String {
    //     format!("{} {}", self.name, self.age)
    // }
}

/*
    --TODO: 特性做参数
        很多情况下我们需要传递一个函数做参数，例如回调函数、设置按钮时间等。Java中函数必须以接口实现的
      类实例来传递，在Rust中可以通过传递特性参数来实现:
*/

fn output(object: impl Descriptive)
{
    println!("{}", object.describe());
}

/*
    任何实现了Descriptive特性的对象都可以作为这个函数的参数，这个函数没必要了解传入对象有没有其他属性或
  方法，只需要了解传入对象有没有其他属性或方法，只需要了解它一定有Descriptive特性规范的方法就可以了。
*/
fn output_one<T: Descriptive>(object: T)
{
    println!("{}", object.describe());
}

/*
    这是一种风格类似泛型的语法糖，这种语法糖在有多个参数类型均是特性的情况下十分实用:
*/

fn output_two<T: Descriptive>(arg1: T, arg2: T)
{
    println!("{}", arg1.describe());
    println!("{}", arg2.describe());
}

/*
    特性作类型表示时如果涉及多个特性，可以用 + 符号表示，例如:
*/
// fn notify (item: impl Summary + Display)
// fn notify<T: Summary + Display>(item: T)

/*
    --TODO: 
        仅用于表示类型的时候，并不意味着可以在impl块中使用。
        复杂的实现关系可以使用where关键字简化。

    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U)
*/

/*  可以简化成:
fn some_function_2<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug

*/

/*
    Tip: 
        由于需要声明compare函数的第二参数必须与实现该特性的类型相同，所以Self(注意大小写)关键字就代表了当前类型(不是实例)本身
*/
trait Comparable
{
    fn compare(&self, object: &Self) -> i8;
}

fn max<T: Comparable>(array: &[T]) -> &T
{
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i].compare(&array[max_index]) > 0
        {
            max_index = i;
        }
        i += 1;
    }
    &array[max_index]
}

impl Comparable for f64 {
    fn compare(&self, object: &f64) -> i8 
    {
        if &self > &object { 1 }
        else if &self == &object { 0 }
        else { -1 }
    }
}

/*
    --TODO: 特性做返回值:
        特性做返回值只接受实现了该特性的对象做返回值且在同一个函数中所有可能的返回值类型必须完全一样，比如结构体A与结构体B都实现了特性Trait。
*/
fn person() -> impl Descriptive
{
    Person {
        name: String::from("Cali"),
        age: 24
    }
}

/*
    --TODO: 有条件啊实现方法:
        impl功能十分强大，我们就可以用它实现类的方法。但对于泛型来说，有时我们需要区分一下它所属的泛型已经实现的方法来决定它接下来该实现的方法:

    struct A<T> {}

    impl<T: B + C> A<T> {
        fn d(&self) {}
    }

    这段代码声明了A<T>类型必须在T已经实现B和C特性的前提下才能有效实现此impl块。
*/

fn main ()
{
    let cali = Person {
        name: String::from("Cali"),
        age: 24
    };
    println!("{}", cali.describe());

    let arr = [1.0, 3.0, 5.0, 4.0, 2.0];
    println!("maximum of arr is {}", max(&arr));
}