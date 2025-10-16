#[test]
fn compile_fails_for_invalid_usage() {
    let t = trybuild::TestCases::new();

    // Each file below must fail compilation
    t.compile_fail("tests/errors/invalid_syntax.rs");
    t.compile_fail("tests/errors/missing_infer_keyword.rs");
    t.compile_fail("tests/errors/bad_literal.rs");
    t.compile_fail("tests/errors/not_named_fields.rs");
    t.compile_fail("tests/errors/not_struct.rs");
}