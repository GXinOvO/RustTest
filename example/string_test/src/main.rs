fn main() {
    let mut s = String::from("foo");
    // push -> 1
    // s.push_str("bar");

    // push -> 2
    let s1 = String::from("bar");
    s.push_str(&s1);

    println!("{}", s);

    let h1 = String::from("Hello, ");
    let h2 = String::from("World!");

    let h3 = h1 + &h2;
    println!("{}", h3);

    let w = "上刊登介绍sdkd";

    for b in w.chars() {
        println!("{}", b);
    }
}
