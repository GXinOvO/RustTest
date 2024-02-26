#[allow(dead_code)]
struct Structure(i32);

fn main() {
  /*
  打印操作由std::fmt里面所定义的一系列宏来处理，包括:
      · format: 将格式化文本写到字符串
      · print!: 与format!类似，但将文本输出到控制台(io::stdout)
      · println!: 与print!类似，但输出结果追加一个换行符
      · eprint!: 与print!类似，但将文本输出到标准错误(io::stderr)
      · eprintln!: 与eprint!类似，但输出结果追加到一个换行符
    这些宏都以相同的做法解析文本。有个额外优点是格式化的正确性会在编译时检查

    std::fmt包含多种trait(特质)来控制文字限制，其中最重要的两种trait基本形式如下:
      · fmt::Debug: 使用{:?}标记。格式化文本以供调试使用
      · fmt::Display: 使用{}标记。以更优雅和友好的风格来格式化文本
    */
  println!("Hello, world!");
      
  // -> 可以使用位置参数
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // -> 可以使用命名参数
  println!("{subject} {verb} {object}", 
          object = "the lazy dog",
          subject = "the quick brown fox",
          verb = "jumps over");

  // -> 可以在`:`后面指定特殊的格式
  println!("{} of {:b} people know binary, the other half don't", 1, 2);

  // -> println!会检查使用到的参数数量是否正确
  // println!("My name is {0}, {1} {0}", "Bond");

  // -> 创建一个结构体Structure 但像结构体这样的自定义类型需要更复杂的方式来处理。
  // println!("This struct `{}` won't print...", Structure(3));

}
