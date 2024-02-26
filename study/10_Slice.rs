/*
    切片(Slice)是对数据值的部分引用
    切片这个名字往往出现在生物课上，我们做样本玻片的时候要从生物体上获取切片，以供在显微镜上观察。在Rust中，
  切片的意思大致也是这样，只不过它从数据取材引用。
*/  

fn main()
{
    /* --TODO: 字符串切片 */
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{} = {} + {}", s, part1, part2);

    /*
        Rust中的字符串类型实质上记录了字符在内存中的起始位置和长度。
        使用..表示范围的语法，两边可以没有运算数:
            ..y 等价于 0...y
            x.. 等价于位置x到数据结束
            ..  等价于位置0到结束
        
        到目前为止，尽量不要在字符串中使用非英文字符，因为编码的问题。被切片引用的字符串禁止更改其值。
     */
    let mut a = String::from("runoob");
    let slice = &a[0..3];
    /*
        a被部分引用，禁止更改其值.
    a.push_str("yes!")

        在Rust中有两种常用的字符串类型: str和String。str是Rust核心语言类型，就是一直在讲的字符串切
      片(String Slice)，常常以引用的形式出现(&str)。
        凡是用双引号包括的字符串常量整体的类型性质都是&str。 
        比如: let s = "hello";

        String类型是Rust标准公共库提供的一种数据类型，他的功能更完善 -- 它支持字符串的追加、清空等
      实用的操作。String和str除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量属性
        String和str都支持切片，切片的结果是&str类型的数据。。

    注意:
        切片结果必须是引用类型，但开发者必须自己明示这一点

    有一个很快的方式可以将String转换成&str
        let s1 = String::from("hello");
        let s2 = &s1[..];

     */
    println!("slice = {}", slice);

    /* --TODO: 非字符串切片 */
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter()
    {
        println!("{}", i);
    }
}