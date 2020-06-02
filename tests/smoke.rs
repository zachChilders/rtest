use wd_40::*;

#[derive(PartialEq, Debug)]
struct TestCase {
    a: i32,
    b: String,
    c: i32,
    d: bool,
}

fn run_contrived_test(input: TestCase) {
    let actual = TestCase {
        a: 23,
        b: String::from("asdf"),
        c: 5,
        d: true,
    };
    assert_eq!(input, actual)
}

generate_suite!(
    run_contrived_test,
    TestCase {
        a: 3,
        b: String::from("asdf"),
        c: 5,
        d: false
    },
    TestCase {
        a: 23,
        b: String::from("asdf"),
        c: 5,
        d: true
    }
);
