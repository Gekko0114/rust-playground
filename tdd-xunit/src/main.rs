use tdd_xunit::test_libs::{TestCaseTest, WasRun};

fn main() {
    let mut test_case_test=  TestCaseTest{test: Box::new(WasRun{..Default::default()})};
    test_case_test.test_template_method();
    test_case_test.test_result();
    test_case_test.test_failed_result_formatting();
    test_case_test.test_failed_result();
}
