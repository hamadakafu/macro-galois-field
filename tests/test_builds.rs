#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/builds/error_expand.rs");
    t.compile_fail("tests/builds/error_expand2.rs");
    t.pass("tests/builds/expand.rs");
}