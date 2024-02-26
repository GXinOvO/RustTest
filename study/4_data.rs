fn main()
{
    // float
    let x = 2.0;    // f64
    let y : f32 = 3.0;  // f32
    println!("x: {}, y: {}", x, y);

    let sum = 5 + 10;   // 加
    let difference = 95.5 - 4.3;    // 减
    let product = 4 * 30;   // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余
    println!("sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}", sum, difference, product, quotient, remainder);

    /*
        Rust不支持++和--,因为这两个运算符出现在变量得前后会影响代码可读性，减弱了开发者对变量改变得意识能力。
     */
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup.x: {}, tup.y: {}, tup.z: {}", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];
    let b = ["January", "February", "March"];
    
    // c是一个长度为5的i32数组
    let c : [i32; 5] = [1, 2, 3, 4, 5];

    // d等同于[3, 3, 3, 3, 3];
    let d = [3; 5];

    let first = a[0];
    let second = a[1];
    
    // a[0] = 123; // error
    let mut a = [1, 2, 3];
    a[0] = 4;
}