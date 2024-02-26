/*
    计算机程序必须在运行时管理他们所使用的内存资源
    大多数的编程语言都有管理内存的功能:
        C/C++这样的语言主要通过手动方式管理内存，开发者需要手动的申请和释放内存资源。
        Java语言编写的程序在虚拟机(JVM)中运行，JVM具备自动回收内存资源的功能。但这种方式常常会降低
      运行时效率，所以JVM会尽可能少的回收资源，这样也会使程序占用较大的内存资源。
        所有权概念是为了让Rust在编译阶段更有效地分析内存资源的有用性以实现内存管理而诞生的概念

    所有权有以下三条规则:
        · Rust中的每个值都有一个变量，称为其所有者。
        · 一次只能有一个所有者。
        · 当所有者不在程序运行范围时，该值将被删除
*/

/*
内存分配:
  如果我们定义了一个变量并给他赋予一个值，这个变量的值存在于内存中。这种情况很普遍。但如果我们
需要储存的数据长度不确定(比如用户输入的一串字符串)，我们就无法在定义时明确数据长度，也就无法在
编译阶段令程度分配固定长度的内存空间供数据储存使用。这就需要提供一种在程序运行时程序自己申请使用
内存的机制 -- 堆。
  有分配就有释放，程序不能一直占用某个内存资源。因此决定资源是否浪费的关键因素就是资源有没有及时
释放。

  Rust之所以没有明示释放的步骤是因为在变量范围结束的时候，Rust编译器自动添加了调用释放资源函数
的步骤。
*/


/*
  --TODO: 变量与数据交互的方式
    1. 移动
      多个变量可以在Rust中以不同的方式与相同的数据交互
      let x = 5;
      let y = x;
    这个程序将值5绑定到变量x，然后将x的值复制并复制给变量y。现在栈中将有两个值5.此情况中的数据时
  "基本数据"类型的数据，不需要存储到堆中，仅在栈中的数据的"移动"方式是直接复制，这不会花费更长的时
  间或更多的存储空间。
  · 所有整数类型: i32、u32、i64等
  · 布尔类型: true或false
  · 所有浮点类型，f32和f64
  · 字符类型char
  · 仅包含以上类型数据的元组(Tuples)

但如果发生在堆上:
  let s1 = String::from("hello");
  let s2 = s1;
  第一步产生一个String对象，值为"hello".其中"hello"可以认为是类似于长度不确定的数据，需要在堆中
存储。
  第二步的情况略有不同:
    两个String对象在栈中，每个String对象都有一个指针指向堆中的"hello"字符串。在给s2赋值时，只有
  栈中的数据被复制了，堆中的字符串依然还是原来的字符串
    之前我们说，当变量超出范围内，Rust自动调用释放资源函数并清理该变量的堆内存。。但是s1和s2都被释放
  的话堆区中的"hello"被释放两次，这是不被系统允许的。为了确保安全，在给s2赋值时s1已经无效了。没错，在
  把s1的值赋给s2以后，s1将不可以再被使用。
*/

fn main()
{
  let s1 = String::from("hello");
  let s2 = s1;
  
  // println!("{}, world!", s1);  //error


  /* --TODO: 克隆
    Rust会尽可能地降低程序的运行成本，所以默认情况下，长度较大的数据存放在堆中，且采用移动的方式进行数据
  交互。但如果需要将数据单纯的复制一份以供他用，可以使用数据的第二种交互方式 -- 克隆

    下面这种方式会真的将堆中的"hello"复制了一份，所以a1和a2都分别绑定了一个值，释放的时候也会被当作两个资源。
    当然，克隆仅在需要复制的情况下使用，毕竟复制数据会花费更多的时间。
   */
  let a1 = String::from("hello");
  let a2 = a1.clone();
  println!("a1 = {}, a2 = {}", a1, a2);

  /* --TODO: 涉及函数的所有权机制 */
  // 声明s有效
  let s = String::from("hello");

  // s的值被当作参数传入函数
  // 所以可以当作 s 已经被移动，从这里开始已经无效
  takes_ownership(s);

  let x = 5;

  // x 的值被当作参数传入函数
  // 但 x 是基本类型，依然有效
  // 在这里依然可以使用 x 却不能使用s
  makes_copy(x);


  /*
    --TODO: 函数返回值的所有权机制
      被当作函数返回值的变量所有权将会被移动出函数并返回到调用函数的地方，而不会直接被无效释放。
   */
  // gives_ownership 移动他的返回值到 b1
  let b1 = gives_ownership();

  // b2 被声明有效
  let b2 = String::from("hello");

  // b2 被当作参数移动，b3获得返回值所有权
  let b3 = takes_and_gives_back(b2);


  /*
    --TODO: 引用与租借
      引用(Reference)是C++开发者较为熟悉
      实质上 引用 是变量的间接访问方式

      & 运算符可以取变量的 引用
      当一个变量的值被引用时，变量本身不会被认定无效。因为 引用 并没有在栈中复制变量的值
    */
  let m1 = String::from("hello");
  let m2 = &m1;
  println!("m1 is {}, m2 is {}", m1, m2);

  let n1 = String::from("hello");
  let len = calculate_lengtha(&n1);
  println!("The length of '{}' is {}", n1, len);


  /*
      引用不会获得值的所有权
      引用只能租借(Borrow)值的所有权
      引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权
   */
  let v1 = String::from("hello");
  let mut v2 = &v1;
  /*  let v3 = v1
      println!("v2 = {}", v2);
      这里是会报错的，因为v2租借的v1已经将所有权移动到了v3，所以v2将无法继续租借使用v1的所有权。
      如果需要使用v2使用该值，必须重新租借
    */
  let v3 = v1;
  v2 = &v3;
  println!("v2 = {}", v2);

  /*
    既然引用不具有所有权，即使他租借了所有权，他也只享有使用权
    如果尝试利用租借来的权利来修改数据会被阻止
   */
  let t1 = String::from("run");
  let t2 = &t1;
  println!("t2 = {}", t2);
  /* 
  这个是不被允许的，租借怎么可以改变房子结构呢
  t2.push_str("oob");
  println!("t2 = {}", t2);
   */

  let mut y1 = String::from("run");
  let y2 = &mut y1;
  y2.push_str("oob");
  println!("y2 = {}", y2);
  /* 
  这里多重可变引用了y1
  Rust 对可变引用的这种设计主要出于对并发状态下发生数据访问碰撞的考虑，在编译阶段就避免了这种
事情的发生。
  由于发生数据访问碰撞的必要条件之一是数据被至少一个使用者写且同事被至少一个其他使用者读或写，所以
在一个值被可变引用时不允许再次被任何引用
  let y3 = &mut y1;
  println!("y2 = {}, y3 = {}", y2, y3);
   */


  /* --TODO: 垂悬引用
    这是一个换了个名字的概念，如果放在有指针概念的变成语言里他就指的是那种没有实际指向一个真正能访问
  的数据的指针(注意，不一定是空指针，还有可能时已经释放的资源)。他们就像失去悬挂物体的绳子。
    垂悬引用在Rust语言里是不允许出现，如果有，编译器会发现他

    这里很显然，伴随着dangle函数的结束，其局部变量的值本省没有被当作返回值，被释放了。但他的引用却被返
  回，这个引用所指向的值已经不能确定的存在，故不允许其出现。
   */
  let reference_nothing = dangle();
} // b3 无效被释放，b2 被移动，b1无效被释放

fn dangle() -> &String 
{
  let s = String::from("hello");

  return &s;
}

fn calculate_lengtha(s: &String) -> usize
{
  return s.len();
}

fn takes_ownership(some_string: String)
{
  // 一个 String 参数 some_string 传入，有效
  println!("{}", some_string);
} // 函数结束，参数some_string在这里释放

fn makes_copy(some_integer: i32)
{
  // 一个 i32 参数 some_integer 传入，有效
  println!("{}", some_integer);
} // 函数结束，参数 some_integer 是基本类型，无需释放


fn gives_ownership() -> String
{
  // some_string 被声明有效
  let some_string = String::from("hello");

  // some_string 被当作返回值移动出函数
  return some_string;
}

fn takes_and_gives_back(a_string: String) -> String
{
  // a_string 被声明有效
  return a_string;
}