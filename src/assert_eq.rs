#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {
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
    #[test]
    #[should_panic(expected = r#"
left:  100
right: 10
"#)]
    fn test_assert_eq_fail() {
        assert_eq!(100, 10);
    }

    #[test]
    fn test_assert_eq_success() {
        assert_eq!(100, 100);
        assert_eq!(100, 100,);
    }
}
