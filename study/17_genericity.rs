/*
    --TODO: Rust泛型与特性
      泛型是一个编程语言不可或缺的机制。
      C++语言中用 模板 来实现泛型，而C语言中没有泛型的机制，这也导致C语言
    难以构建类型复杂的工程。
      泛型机制是编程语言用于表达类型抽象的机制，一般用于功能确定、数据类型
    待定的类，如链表、映射表等。
*/

/*
    --TODO: 在函数中定义泛型
        这是一个对整型数字选择排序的方法:f
*/
fn max(array: &[i32]) -> i32 
{
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() 
    {
      if array[i] > array[max_index]
      {
        max_index = i;
      }
      i += 1;
    }
    return array[max_index];
}

/*
  max可以处理i32数字类型的数据，但无法用于f64类型的数据，因此使用泛型可以使这个函数利用到各个类型中去。
  fn max_T<T>(array: &[T]) -> T
  {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len()
    {
      if array[i] > array[max_index]
      {
        max_index = i;
      }
      i += 1;
    }
    array[max_index]
  }
  */

/*
  --TODO: 结构体与枚举类中的泛型
  之前的Option和Result枚举类就是泛型的。
  Rust中的结构体和枚举类都可以实现泛型机制
*/

struct Point<T> 
{
  x: T,
  y: T
}

struct Pointdouble<T1, T2>
{
  x: T1,
  y: T2
}

/*
  impl关键字的后方必须由<T>，因为它后面的T是以之位榜样的。但我们也可以为其中的一种泛型添加方法：
  impl Point<f64> {
    fn x(&self) -> f64 {
      self.x
    }
  }
*/
impl<T> Point<T>
{
  fn x(&self) -> &T {
    &self.x
  }
}

/*
  impl块本身的泛型并没有阻碍其内部方法具有泛型的能力:

  方法mixup将一个Point<T, U>点的x与Point<V, W>点的y融合成一个类型为Point<T, W>的新点。
*/
impl<T, U> Point<T, U>
{
  fn mixup<V, W>(self, other: Point<V, W>) -> Pount<T, W>
  {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

enum Option<T> 
{
  Some(T),
  None,
}

enum Result<T, E>
{
  Ok(T),
  Err(E),
}

fn main()
{
  let a = [2, 4, 6 , 3, 1];
  println!("max = {}", max(&a));

  let p1 = Point { x: 1, y: 2 };
  let p2 = Pointdouble { x: 1.0, y: 2 };
  println!("p1.x = {}", p1.x());
}