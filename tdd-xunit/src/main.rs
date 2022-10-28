use tdd_xunit::test_libs::{TestCaseTest};

fn main() {
    let test_case_test: TestCaseTest =  Default::default();
    test_case_test.test_running();
    test_case_test.test_set_up();
}
