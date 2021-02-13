#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!(
                r#"
left:  {:#?}
right: {:#?}
"#,
                $left, $right
            )
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = r#"
left:  100
right: 10
"#)]
    fn test_assert_eq() {
        assert_eq!(100, 10);
    }
}
