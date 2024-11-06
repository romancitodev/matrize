use std::path::Path;

#[test]
fn matrize() {
    let t = trybuild::TestCases::new();
    let tests_path = Path::new("./tests/lib");

    // test cases that passes:
    for test in ["01-valid-matrix.rs", "02-valid-manual-matrix.rs"] {
        t.pass(tests_path.join(test));
    }

    for test in ["01-invalid-matrix.rs", "02-invalid-identity.rs"] {
        t.compile_fail(tests_path.join(test));
    }
}
