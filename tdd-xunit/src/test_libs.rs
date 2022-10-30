#[derive(Default, Debug, Clone)]
pub struct WasRun {
    pub log: String
}

impl WasRun {
    pub fn test_method(&mut self) -> Result<usize, &'static str> {
        self.log = self.log.clone() + "testMethod ";
        Ok(1)

    }

    pub fn test_broken_method(&mut self) -> Result<usize, &'static str> {
        Err("error")
    }
}

#[derive(Default)]
pub struct TestResult {
    pub run_count: i32,
    pub error_count: i32
}

impl TestResult {
    pub fn test_started(&mut self) {
        self.run_count += 1;
    }

    pub fn test_failed(&mut self) {
        self.error_count += 1;
    }

    pub fn summary(&self) -> String {
        self.run_count.to_string() + " run, " + &self.error_count.to_string() + " failed"
    }
}

pub trait TestCaseTrait {
    fn run(&mut self, func_name: &str, result: &mut TestResult);
    fn set_up(&mut self);
    fn get_log(&self) -> String;
    fn tear_down(&mut self);
}

impl TestCaseTrait for WasRun {
    fn run(&mut self, func_name: &str, result: &mut TestResult) {
        result.test_started();
        self.set_up();
        let f = match func_name {
            "test_method" => self.test_method(),
            "test_broken_method" => self.test_broken_method(),
            _ => Err("no match functions")
        };        
        let _value = match f {
            Ok(_v) => 1,
            Err(_e) => {
                result.test_failed();
                0
            }
        };
        self.tear_down();
      }

    fn set_up(&mut self) {
        self.log = String::from("SetUp ");
    }

    fn get_log(&self) -> String {
        self.log.clone()
    }

    fn tear_down(&mut self) {
        self.log = self.log.clone() + "tearDown ";
    }
}

pub struct TestCaseTest {
    pub test: Box<dyn TestCaseTrait + 'static>
}

impl TestCaseTest {
    pub fn test_template_method(&mut self) {
        let mut result: TestResult = Default::default();
        self.test.run("test_method", &mut result);
        assert!("SetUp testMethod tearDown " == self.test.get_log());
    }

    pub fn test_result(&mut self) {
        let mut result: TestResult = Default::default();
        self.test.run("test_method", &mut result);
        println!("{}", result.summary());        
        assert!("1 run, 0 failed" == result.summary());
    }

    pub fn test_failed_result(&mut self) {
        let mut result: TestResult = Default::default();
        self.test.run("test_broken_method", &mut result);
        println!("{}", result.summary());        
        assert!("1 run, 1 failed" == result.summary());
    }

    pub fn test_failed_result_formatting(&mut self) {
        let mut result: TestResult = Default::default();
        result.test_started();
        result.test_failed();
        println!("{}", result.summary());        
        assert!("1 run, 1 failed" == result.summary());
    }

    pub fn test_suite(&mut self) {
        let mut suite: TestSuite =  Default::default();
        suite.add("test_method");
        suite.add("test_broken_method");
        let result = suite.run();
        println!("{}", result.summary());
        assert!("2 run, 1 failed" == result.summary());
    }
}

#[derive(Default)]
pub struct TestSuite {
    tests: Vec<String>
}

impl TestSuite {
    pub fn add(&mut self, test_name: &'static str) {
        self.tests.push(String::from(test_name));
    }

    pub fn run(&mut self) -> TestResult {
        let mut result: TestResult = Default::default();
        let mut test_cases=  TestCaseTest{test: Box::new(WasRun{..Default::default()})};
        for test_name in &self.tests {
            test_cases.test.run(test_name, &mut result);
        }
        result
    }
}