use climb_stairs70::climb_stairs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        let nums: i32 = 3;
        let result: i32 = climb_stairs(nums);
        assert_eq!(result, 3)
    }
}
