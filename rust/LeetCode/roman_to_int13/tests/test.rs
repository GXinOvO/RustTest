use roman_to_int13::{
    roman_to_int1,
    roman_to_int2,
};

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_roman_to_int1()
    {
        let s = "III";
        let result: i32 = roman_to_int1(s.to_string());
        assert_eq!(result, 3);
        let s = "IV";
        let result: i32 = roman_to_int1(s.to_string());
        assert_eq!(result, 4);
        let s = "IX";
        let result: i32 = roman_to_int1(s.to_string());
        assert_eq!(result, 9);
        let s = "LVIII";
        let result: i32 = roman_to_int1(s.to_string());
        assert_eq!(result, 58);
        let s = "MCMXCIV";
        let result: i32 = roman_to_int1(s.to_string());
        assert_eq!(result, 1994);
    }

    #[test]
    fn test_roman_to_int2()
    {
        let s = "III";
        let result: i32 = roman_to_int2(s.to_string());
        assert_eq!(result, 3);
        let s = "IV";
        let result: i32 = roman_to_int2(s.to_string());
        assert_eq!(result, 4);
        let s = "IX";
        let result: i32 = roman_to_int2(s.to_string());
        assert_eq!(result, 9);
        let s = "LVIII";
        let result: i32 = roman_to_int2(s.to_string());
        assert_eq!(result, 58);
        let s = "MCMXCIV";
        let result: i32 = roman_to_int2(s.to_string());
        assert_eq!(result, 1994);
    }
}