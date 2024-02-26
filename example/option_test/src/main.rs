/*
    --TODO: Option
enum Option<T> {
    Some(T),
    None,
}

*/

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let v = Some(0u8);
    match v {
        Some(1) => println!("One"),
        Some(2) => println!("Two"),
        _ => println!("Others"),
    }

    if let Some(2) = v {
        println!("Two");
    } else {
        println!("Others");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> 
{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
