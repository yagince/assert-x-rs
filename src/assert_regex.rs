#[macro_export]
macro_rules! assert_regex {
    ($actual:expr, $expect:expr $(,)?) => {
        match regex::Regex::new($expect) {
            Err(e) => {
                panic!("{}", e)
            }
            Ok(reg) => {
                if !reg.is_match($actual) {
                    panic!(
                        r#"
actual: {}
regex:  /{:#?}/
"#,
                        $actual, reg
                    )
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assert_regex_success() {
        assert_regex!("hoge", "oge");
        assert_regex!("abcdefghijklmn", "bcdefghijklm",);
    }

    #[test]
    #[should_panic(expected = r#"
actual: hoge
regex:  /aiueo/
"#)]
    fn test_assert_regex_fail() {
        let hoge = "hoge".to_string();
        assert_regex!(&hoge, "aiueo");
    }

    #[test]
    #[should_panic(expected = r#"regex parse error:
    aiu[?o
       ^
error: unclosed character class"#)]
    fn test_assert_regex_invalid_pattern() {
        assert_regex!("hoge", "aiu[?o");
    }
}
