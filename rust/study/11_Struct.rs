/*
    --TODO: Struct
        Rust中struct语句仅用来定义，不能声明实例，结尾不需要 ; 符号，而且每个字段定义之后用 , 分隔。

    结构体类名 {
        字段名: 字段值,
        ...
    }
*/


struct Site
{
    domain: String,
    name: String,
    nation: String,
    found: u32
}

#[derive(Debug)]    // 在println中就可以用{:?}占位符输出一整个结构体
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 
    {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool
    {
        self.width > rect.width
    }

    fn create(width: u32, height: u32) -> Rectangle
    {
        Rectangle { 
            width, 
            height 
        }
    }
}

/*
    --TODO: 单元结构体
        结构体可以只作为一种象征而无需任何成员
*/
struct UnitStruct;

fn main()
{
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain,
        name,
        nation: String::from("China"),
        found: 2023
    };

    // 更新struct构造实例
    let site = Site {
        domain: String::from("www..runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob
    };

    /* --TODO: 元组结构体 */
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);

    /*
        --TODO: 结构所有权
            结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段，
     */
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    println!("rect1 is {:#?}", rect1);

    /*
        --TODO: 结构体方法
            方法和函数类似，只不过他是用来操作结构体实例的。
            结构体方法的第一个参数必须是
     */
    println!("rect1's area is {}", rect1.area());
    
    let rect2 = Rectangle {
        width: 40,
        height: 20
    };
    println!("{}", rect1.wider(&rect2));

    /*
        --TODO: 结构体关联函数
            其在impl块中却没有&self参数。
            这种函数不依赖实例，但是使用它需要声明是在哪个impl块中的。
        一种使用的 String::from 函数就是一个"关联函数".
     */
    let rect3 = Rectangle::create(30, 50);
    println!("rect3: {:?}", rect3);

    /* --TODO: 结构体impl块可以写几次，效果相当于他们内容的拼接 */
}   