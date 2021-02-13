#[test]
#[should_panic(expected = r#"
left:  100
right: 10
"#)]
fn test_assert_eq() {
    assert_x::assert_eq!(100, 10);
}
