use wd_40::generate_suite;

/// Test name is derived from the name of your struct.  
/// Having a struct to handle input is mandatory
#[derive(PartialEq, Debug)]
struct TestCase {
    a: i32,
    b: String,
    c: i32,
    d: bool,
    should_fail: bool
}

/// Test function to run against all test cases
/// It _must_ be of this type:
/// `Fn(input: T);`
fn run_contrived_test(input: TestCase) {
    // We construct a case here, but IRL you may want a helper function to return your data type.
    let actual = TestCase {
        a: 23,
        b: String::from("asdf"),
        c: 5,
        d: true,
        should_fail: false,
    };

    if !input.should_fail {
        assert_eq!(input, actual)
    } else {
        assert_ne!(input, actual)
    }
}

generate_suite!(
    run_contrived_test,
    TestCase {
        a: 3,
        b: String::from("asdf"),
        c: 5,
        d: false,
        should_fail: true
    },
    TestCase {
        a: 23,
        b: String::from("asdf"),
        c: 5,
        d: true,
        should_fail: false
    }
);
