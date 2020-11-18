use std::i32;

pub fn atoi(s: &str) -> i32 {
    if s.trim().is_empty() {
        return 0;
    }
    match s.split_whitespace().collect::<Vec<_>>()[0].parse::<f64>() {
        Ok(val) => match val {
            d if d < (i32::MIN as f64) => i32::MIN,
            d if d > (i32::MAX as f64) => i32::MAX,
            _ => val as i32,
        },
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(atoi("42"), 42);
    }
    #[test]
    fn test2() {
        assert_eq!(atoi("-42"), -42);
    }
    #[test]
    fn test3() {
        assert_eq!(atoi("      42"), 42);
    }
    #[test]
    fn test4() {
        assert_eq!(atoi("4193 with words"), 4193);
    }
    #[test]
    fn test5() {
        assert_eq!(atoi("words and 987"), 0);
    }
    #[test]
    fn test6() {
        assert_eq!(atoi("-91283472332"), i32::MIN);
    }
    #[test]
    fn test7() {
        assert_eq!(atoi("91283472332"), i32::MAX);
    }
    #[test]
    fn test8() {
        assert_eq!(atoi(""), 0);
    }
    #[test]
    fn test9() {
        assert_eq!(atoi("    "), 0);
    }
    #[test]
    fn test10() {
        assert_eq!(atoi("13.73"), 13);
    }
    #[test]
    fn test11() {
        assert_eq!(atoi("-00013.73"), -13);
    }
    #[test]
    fn test12() {
        assert_eq!(atoi("+00013.73"), 13);
    }
    #[test]
    fn test13() {
        assert_eq!(atoi("1337leet"), 1337);
    }
}
