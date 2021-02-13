#[test]
#[should_panic(expected = r#"
actual: 100
regex:  /11/
"#)]
fn assert_regex_oneline() {
    assert_x::assert_regex!("100", "11");
}
