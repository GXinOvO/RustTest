pub mod math;

pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64)
{
    if b == 0
    {
        (a.abs(), a.signum(), 0)
    } else 
    {
        let (d, coef_b, coef_a) = extended_gcd(b, a % b);
        (d, coef_a, coef_b - coef_a * (a / b))
    }
}

fn main()
{
    let (a, b) = (14, 35);
    
    let (d, x, y) = extended_gcd(a, b);
    println!("d: {}", d);
    assert_eq!(d, 7);
}