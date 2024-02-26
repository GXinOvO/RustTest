/// predict if two matrices contains same value
/* --TODO: 比较两个切片是否相等 */
// -> wher T: Eq 约束，表示类型T必须实现Eq trait，即可进行相等比较。
pub fn equal_slice<T>(a: &[T], b: &[T]) -> bool where T: Eq {
  // -> 比较两个切片a和b的长度是否相等，如果长度不相等，则直接返回false，表示切片不相等。
  if a.len() != b.len() {
    return false;
  } else {
    // -> 如果长度相等，函数使用a.iter()返回一个迭代器，然后使用enumerate()方法对迭代器进行遍历，获取每个元素的索引i和对应的值x。
    for (i, x) in a.iter().enumerate() {
      if *x != b[i] {
        return false;
      }
    }
  }

  true
  
}