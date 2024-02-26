/*
--TODO: List_test
    对一个结构体实现fmt::Display，其中的元素需要一个接一个地处理到，这可能会很麻烦。
  问题在于每个write!都要生成一个fmt::Result.    ?
*/
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate()
        {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

fn main()
{
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}