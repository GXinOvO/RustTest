enum SpreadsheetCell 
{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => {
    //         println!("The third element is {}", third)
    //     },
    //     None => {
    //         println!("There is no third element")
    //     },
    // }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
