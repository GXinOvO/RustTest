/*
--TODO: where
    约束也可以使用where分句来表达，他放在{的前面，而不需写在类型第一次出现之前。另外
  where从句可以用于任意类型的限定，而不局限于类型参数本身。
    where在下面一些情况下很有用:
*/

// -> 1· 当分别指定泛型的类型和约束会更清晰时:
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YoutType {}

// -> 使用`where`从句来表达约束
impl <A, D> MyTrait<A, D> for YoutType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}

// -> 2· 当使用where从句比正常语法更有表现力时。

use std::fmt::Debug;

trait PrintInOption
{
    fn print_in_option(self);
}

impl<T> PrintInOption for T where
    Option<T>: Debug {
        fn print_in_option(self)
        {
            println!("{:?}", Some(self));
        }
    }

fn main()
{
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}