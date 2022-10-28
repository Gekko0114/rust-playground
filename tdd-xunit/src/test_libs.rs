#[derive(Default, Debug, Clone, Copy)]
pub struct TestCase<'a> {
    pub name: &'a str
}

#[derive(Default, Debug, Clone, Copy)]
pub struct WasRun<'a> {
    pub was_run: i32,
    pub testcase: TestCase<'a>,
    pub was_set_up: i32,
    pub log: &'a str
}

impl <'a>WasRun<'a> {
    pub fn run(&mut self) {
      match self.testcase.name {
        "test_method" => self.test_method(),
        "set_up" => self.set_up(),
        _ => return
      };
    }

    pub fn test_method(&mut self) {
        self.was_run = 1;
    }

    pub fn set_up(&mut self) {
        self.was_run = 0;
        self.was_set_up = 1;
        self.log = "SetUp ";
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct TestCaseTest {
}

impl TestCaseTest {
    pub fn test_running(&self) {
        let mut test =  WasRun{ testcase:{ TestCase{name: "test_method"}}, ..Default::default() };
        test.run();
        assert!(test.was_run == 1);
    }

    pub fn test_set_up(&self) {
        let mut test: WasRun = WasRun{ testcase:{ TestCase{name: "set_up"}}, ..Default::default()  };
        test.run();
        assert!(test.was_set_up == 1);
        assert!("SetUp " == test.log);
    }
}