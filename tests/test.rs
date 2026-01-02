#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/*.rs");
    t.compile_fail("tests/ui-fail/*.rs");
}