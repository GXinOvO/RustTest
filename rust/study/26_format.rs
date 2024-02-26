/*
——TODO: formatting
    格式化的方式是通过格式字符串来指定的:
        · format!("{}", foo) -> "3735928559"
        · format!("0x{:X}", foo) -> "0xDEADBEEF"
        · format!("0o{:o}", foo) -> "0o33653337357"
    
    这个格式化的功能是通过trait实现的，每种参数类型都对应一种trait。最常见
  的格式化trait就是Display，它可以处理参数类型为未指定的情况，比如{}
*/
use std::fmt::{self, Formatter, Display};

struct City
{
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City
{
    // -> f是一个缓冲区(buffer)，此方法必须将格式化后的字符串写入其中
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // -> write!和format!类似，但他会将格式化后的字符串写入一个缓冲区(即第一个参数f中)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
                self.name , self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color
{
    red: u8,
    green: u8,
    blue: u8,
}

fn main()
{
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.24, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0},
    ].iter() {
        println!("{:?}", *color);
    }

    // -> 通过加前缀0x、0o、0b，数字可以用十六进制、八进制或二进制记法表示
}