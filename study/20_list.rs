/*
    --TODO: Rust集合与字符串
        集合(Collection)是数据结构中最普遍的数据存放形式，Rust标准库中提供了丰富的集合类型
      帮助开发者处理数据结构的操作。

    --TODO: 向量
        向量(Vector)是一个存放多值的单数据结构，该结构将相同类型的值线性的存放在内存中。
        向量是线性表，在Rust中的表示是Vec<T>
        向量的使用方式类似于列表(List)，我们可以通过以下方式创建指定类型的向量:

    let vector: Vec<i32> = Vec::new();
    let vector = vec![1, 2, 3, 4];
*/

use std::collections::HashMap;

// -> 使用线性表常常会用到追加操作，但是追加和栈的push操作本质是一样的，所以向量只有push方法来追加单个元素:
fn main()
{
    let mut vector = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    vector.push(64);
    println!("{:?}", vector);

    /*
        --TODO: 
            append方法用于将一个向量拼接到另一个向量的尾部
     */
    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("{:?}", v1);

    /*
        --TODO:
            get方法用于取出向量中的值

            因为向量的长度无法从逻辑上推断，get方法无法保证一定取到值，所以get方法的返回值是Option枚举类，有可能为空。
          这是一种安全的取值方法，但是书写起来有些麻烦。如果能够保证取值的下标不会超出向量下标取值范围，也可以使用
          数组取值语法
     */
    let mut v = vec![1, 2, 4, 8];
    println!("{}", match v.get(2) {
        Some(value) => value.to_string(),
        None => "None".to_string()
    });

    let b = vec![1, 2, 4, 8];
    println!("{}", v[1]);

    /*
        --TODO:
            如果遍历过程中需要更改变量的值
     */
    let mut z = vec![100, 32, 57];
    for i in &mut z 
    {
        *i += 50;
    }
    println!("z = {:?}", z);


    /*
        --TODO: 字符串
     */
    let string = String::new();

    // -> 基础类型转换成字符串
    let one = 1.to_string();            // 整数到字符串
    let float = 1.3.to_string();        // 浮点数到字符串
    let slice = "slice".to_string();    // 字符串切片到字符串    

    let mut s = String::from("run");
    s.push_str("oob");
    s.push('!');

    println!("s = {}", s);

    /*
        --TODO: 用 + 号拼接字符串:
     */
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;

    let s4 = s1.clone() + "-" + &s2 + "-" + &s3;
    println!("s4 = {}", s4);

    /*
        --TODO: 使用format! 宏
     */
    let a1 = String::from("tic");
    let a2 = String::from("tac");
    let a3 = String::from("toe");

    let a = format!("{}-{}-{}", a1, a2, a3);
    println!("a = {}", a);

    /*
        --TODO: 字符串长度
     */
    let d = "hello";
    let len = d.len();
    println!("d = {}", len);

    /*
        注意:
            这里的lenq的值是6.因为中文是UTF-8编码的，每个字符长3个字节，所以长度为6。
     */
    let q = "你好";
    let lenq = q.len();
    println!("q = {}", lenq);

    /*
        Rust中支持UTF-8字符对象，所以如果想统计字符数量可以先取字符串为字符集合。
        这里len的值是7，因为一共有7个字符。统计字符的速度比统计数据长度的速度慢得多
     */
    let q2 = "hello你好";
    let lenq2 = q2.chars().count();
    println!("q2 = {}", lenq2);

    for c in q2.chars()
    {
        println!("{}", c);
    }

    /*
        --TODO: 从字符串中取单个字符
            nth函数是从迭代器中取出某值得方法，请不要再遍历中这样使用! 因为UTF-8每个字符的长度不一定相等
     */
    let o = q2.chars().nth(2);
    println!("{:?}", o);

    let sub = &q2[0..3];
    println!("{}", sub);

    /*
        --TODO: 映射表
            映射表(Map)在其他语言中广泛存在。其中应用最普遍的就是键值散列映射表(Hash Map)。
     */
    let mut map = HashMap::new();

    map.insert("color", "red");
    map.insert("size", "10 m^2");

    /*
        注意:
            这里没有声明散列表的泛型，是因为Rust的自动判断类型机制
     */
    println!("{}", map.get("color").unwrap());

    /*
        insert方法和get方法是映射表最常用的两个方法。
        映射表支持迭代器
     */
    for p in map.iter()
    {
        println!("{:?}", p);
    }

    /*
        迭代元素是表示键值对的元组
        Rust的映射表是十分方便的数据结构，当使用insert方法添加新的键值对的时候，如果
      已经存在相同的键，会直接覆盖对应的值。如果你想"安全地插入",就是在确认当前不存在某个键
      时才执行的插入动作:
    
        这句话的意思时如果没有键为"colot"的键值对就添加他并设定值为"red"，否则将跳过。
     */
    map.entry("color").or_insert("red");

    /*
        在已经确定有某个键的情况下如果想直接修改对应的值，有更快的办法:
     */
    map.insert("str", "a");
    if let Some(x) = map.get_mut(&"str")
    {
        *x = "b";
    }
}