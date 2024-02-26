pub fn is_palindrome1(
    x: i32,
) -> bool
{
    if x >= 0 && x < 10 { return true; }
    if x < 0 || x % 10 == 0 { return false; }

    let mut x1: i32 = x;
    let mut half: i32 = 0;

    loop
    {
        if x1 <= half { break; }
    
        half = half * 10 + x1 % 10;
        x1 = x1 / 10;
    }
    if x1 == half || x1 == half / 10 { return true; }
    false
}

pub fn is_palindrome2(
    x: i32,
) -> bool
{
    if x < 0 { return false; }
    if x >= 0 && x< 10 { return true; }
    let y = x.to_string();
    for i in 0..y.len()
    {
        if &y[i..i+1] == &y[y.len()-i-1..y.len()-i]
        {
            if i > (y.len() - i - 1) { return true; }
        }
        else 
        {
            return false
        }
    }
    true
}