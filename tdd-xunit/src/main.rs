use tdd_xunit::wasrun::{WasRun, TestCase};

fn main() {
    let mut test =  WasRun{ testcase:{ TestCase{name: "test_method"}}, ..Default::default() };
    println!("{}", test.call_was_run());
    test.run();
    println!("{}", test.call_was_run());
}
