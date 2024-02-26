/*
    --TODO: 别名
        可以用type语句给已有的类型取个新的名字。
*/

type NanoSecond = u64;
type Inch = u64;

// -> 通过这个属性屏蔽警告
#[allow(non_camel_case_types)]
type u64_t = u64;

fn main()
{
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?",
            nanoseconds,
            inches,
            nanoseconds + inches);

}