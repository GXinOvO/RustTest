const MAX_POINTS: u32 = 100_000;

fn main() {
    // println!("Hello, world!");

    // let mut x = 5;
    // println!("The value of x is {}", x);

    // x = 6;
    // println!("The value os x is {}", x);

    let s = String::from("Hello World");
    take_ownership(s);

    let x = 5;
    makes_copy(x);

    println!("x: {}", x);

    let a = String::from("Hello World!");

    let word_index = first_world(&a);

    println!("{}", word_index);

}

fn take_ownership(some_string: String)
{
    println!("{}", some_string)
}

fn makes_copy(some_number: i32)
{
    println!("{}", some_number)
}


fn first_world(s: &String) -> usize
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}