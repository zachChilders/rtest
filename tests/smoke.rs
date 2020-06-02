use rtest::*;

#[derive(PartialEq, Debug)]
struct Test {
    a: i32,
    b: String,
    c: String,
    d: bool,
}

fn run_my_test(expected: Test) {
    let actual = Test {
        a: 23,
        b: String::from("asdf"),
        c: String::from("aeou"),
        d: true,
    };
    assert_eq!(expected, actual)
}

describe_suite!(
    run_my_test,
    Test {
        a: 3,
        b: "asdf",
        c: "aeou",
        d: false
    },
    Test {
        a: 23,
        b: "asdf",
        c: "asdf",
        d: true
    }
);
