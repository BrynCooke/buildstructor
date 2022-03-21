#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/buildstructor/fail/*.rs");
    t.pass("tests/buildstructor/pass/*.rs");
}
