fn next(mut n: i32) -> i32 {
    let mut sum = 0;
    while n != 0 {
        let a = n % 10;
        sum += a * a;
        n /= 10;
    }
    return sum;
}

pub fn is_happy(n: i32) -> bool {
    let (mut a, mut b) = (n, n);
    loop {
        a = next(a);
        if a == 1 {
            return true;
        }

        a = next(a);
        b = next(b);
        if a == b {
            return false;
        }
    }
}
