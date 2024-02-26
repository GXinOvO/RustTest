use is_palindrome9::{
    is_palindrome1,
    is_palindrome2,
};

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_is_palindrome1()
    {
        let x: i32 = 121;
        let result: bool = is_palindrome1(x);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_palindrome2()
    {
        let x: i32 = 121;
        let result: bool = is_palindrome2(x);
        assert_eq!(result, true);
    }
}