#[derive(Default, Debug, Clone, Copy)]
pub struct TestCase<'a> {
    pub name: &'a str
}

#[derive(Default, Debug, Clone, Copy)]
pub struct WasRun<'a> {
    pub was_run: i32,
    pub testcase: TestCase<'a>
}

impl <'a>WasRun<'a> {
    pub fn run(&mut self) {
      match self.testcase.name {
        "test_method" => self.test_method(),
        _ => return
      };
    }

    pub fn test_method(&mut self) {
        self.was_run = 1;
    }

    pub fn call_was_run(&self) -> i32 {
        self.was_run
    }
}