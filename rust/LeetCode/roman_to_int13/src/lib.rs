pub fn roman_to_int1(
    s: String
) -> i32
{
    fn roman_to_init_char(
        c: char
    ) -> Option<i32>
    {
        match c
        {
            'I' => Some(1),
            'V' => Some(5),
            'X' => Some(10),
            'L' => Some(50),
            'C' => Some(100),
            'D' => Some(500),
            'M' => Some(1000),
            _   => None,
        }
    }

    let mut v = 0i32;
    if s.is_empty() { return 0; }

    s.chars().zip(s.chars().skip(1)).for_each(
        |(first, second)|
        {
            let a = roman_to_init_char(first).unwrap();
            let b = roman_to_init_char(second).unwrap();
            v += if a < b { -1 * a } else { a };
        }
    );
    v += roman_to_init_char(s.chars().last().unwrap()).unwrap();
    v
}

pub fn roman_to_int2(
    s: String
) -> i32
{
    let v: Vec<_> = s.chars().collect();
    let mut index: usize = 0;
    let mut result: i32 = 0;
    let mut pre: i32 = 0;
    while index < v.len()
    {
        let mut current: i32 = 0;
        match v[index] 
        {
            'V' => current = 5,
            'L' => current = 50,
            'D' => current = 100,
            'M' => current = 500,
            'I' => current = 1,
            'X' => current = 10,
            'C' => current = 1000,
            _   => current = 0,
        }

        result += current;

        if current > pre && pre != 0 { result -= 2 * pre; }

        pre = current;
        index += 1;
    }
    result
}