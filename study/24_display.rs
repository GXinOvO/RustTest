/*
--TODO: 显示
    fmt::Debug通常看起来不太简介，因此自定义输出的外观经常使更可取的。这需要通过手动
  实现fmt::Display来做到。fmt::Display采用{}标记。
*/

// -> 使用use导入fmt模块使 fmt::Display 可用
use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure
{
    // 这个trait要求`fmt`使用与下面的函数完全一致签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        // 仅将self的第一个元素写入到给定的输出流f。返回fmt::Result，此
        // 结果表明操作成功或失败
        write!(f, "{}", self.0)
    }
}

// -> 对于Vec<T> 或其他任意泛型容器，fmt::Display都没有实现。在这些情况下要用fmt::Debug

#[derive(Debug)]
struct MinMax(i64, i64);

// -> 实现MinMax的Display
impl fmt::Display for MinMax
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // 使用self.number来表示各个数据
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D
{
    x: f64,
    y: f64,
}

// -> 类似地对Point2D实现Display
impl fmt::Display for Point2D
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main()
{
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
            small = small_range,
            big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}