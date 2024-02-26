/*
    --TODO: use关键字
        use关键字能够将模块标识符引入当前作用域
*/

mod nation
{
    pub mod government
    {
        pub fn govern() {}
    }
    /* 当然，有些情况下存在两个相同的名称，且同样需要导入，我们可以使用as关键字为标识符添加别名 */
    pub fn govern() {}

    /*
        use关键字可以与pub关键字配合使用
        pub use government::govern;
     */
}

use crate::nation::government::govern;
use crate::nation::govern as nation_govern;

/*
    --TODO: 引用标准库
        所有的系统库模块都是被默认导入的，所以在使用的时候只需要使用use关键字简化
      路径就可以方便的使用了
*/
use std::f64::consts::PI;

fn main()
{
    /*
        因为use关键字把govern标识符导入到了当前的模块中，可以直接使用。
        这样就解决了局部模块路径过长的问题。
     */
    govern();

    nation_govern();

    /* use + pub
        nation::govern();
     */


    println!("{}", (PI / 2.0).sin());
}

