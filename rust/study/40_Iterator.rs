/*
--TODO: Iterator::any
    Iterator::any是一个函数，若传给它一个迭代器(iterator)，当其中任一元素满足谓词(predicate)
  时他将返回true，否则返回false(译注: 谓词时闭包规定的, true/false时闭包作用在元素上的返回值)。
*/
pub trait Iterator
{
    // -> 被迭代的类型
    type Item;

    // -> `any`接受`&mut self`参数
    // -> 表明函数的调用者可以被借用和修改，但不会被消耗
    fn any<F>(&mut self, f: F) -> bool where
        // -> `FnMut`表示被捕获的变量最多只能被修改，而不能被消耗
        // -> `Self::Item`表明变量是通过值传递给闭包(译注: 是迭代器对应的元素的类型)
        F: FnMut(Self::Item) -> bool {}
}

fn main()
{
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    /*
        译注: `any`方法会自动地把`vec.iter()`举出的迭代器的元素一个个地传给闭包。
     */
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    
    // -> 对vec的`into_iter()`举出`i32`类型。无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // let array1 = [1, 2, 3];
    // let array2 = [4, 5, 6];

    // println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}