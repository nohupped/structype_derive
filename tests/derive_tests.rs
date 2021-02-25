#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/parse_pass.rs"); // We are using this test just to make sure this is parsed.
}
