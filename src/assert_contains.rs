#[macro_export]
macro_rules! assert_contains {
    ($actual:expr, $expect:expr $(,)?) => {
        let extra = $expect
            .iter()
            .cloned()
            .filter(|x| !$actual.contains(&x))
            .collect::<Vec<_>>();

        if extra.len() > 0 {
            panic!(
                r#"
Some values are not contained.
------------------------------
actual:
{:?}
------------------------------
expected:
{:?}
------------------------------
not contained:
{:?}
"#,
                &$actual, &$expect, extra
            )
        }
    };
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, PartialEq)]
    struct Hoge {
        a: String,
        b: i32,
    }

    #[test]
    fn test_assert_contains_success() {
        assert_contains!(vec![1, 2, 3], [2, 3]);
    }

    #[test]
    fn test_assert_contains_success_struct() {
        assert_contains!(
            vec![
                Hoge {
                    a: "hoge".into(),
                    b: 100,
                },
                Hoge {
                    a: "fooo".into(),
                    b: 200,
                }
            ],
            [Hoge {
                a: "fooo".into(),
                b: 200,
            }]
        );
    }

    #[test]
    #[should_panic(expected = r#"
Some values are not contained.
------------------------------
actual:
[1, 2, 3]
------------------------------
expected:
[4, 3, 2, 0]
------------------------------
not contained:
[4, 0]
"#)]
    fn test_assert_contains_fail() {
        assert_contains!(vec![1, 2, 3], [4, 3, 2, 0]);
    }

    #[test]
    #[should_panic(expected = r#"
Some values are not contained.
------------------------------
actual:
[Hoge { a: "hoge", b: 100 }, Hoge { a: "fooo", b: 200 }]
------------------------------
expected:
[Hoge { a: "not contained", b: 200 }]
------------------------------
not contained:
[Hoge { a: "not contained", b: 200 }]
"#)]
    fn test_assert_contains_fail_struct() {
        assert_contains!(
            vec![
                Hoge {
                    a: "hoge".into(),
                    b: 100,
                },
                Hoge {
                    a: "fooo".into(),
                    b: 200,
                }
            ],
            [Hoge {
                a: "not contained".into(),
                b: 200,
            }]
        );
    }
}
