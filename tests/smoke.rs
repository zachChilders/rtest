use rtest::*;

#[derive(PartialEq, Debug)]
struct Test {
    a: i32,
    b: i32,
    c: i32,
    d: bool,
}

fn run_my_test(expected: Test) {
    let actual = Test {
        a: 23,
        b: 5,
        c: 5,
        d: true,
    };
    assert_eq!(expected, actual)
}

describe_suite!(
    run_my_test,
    Test {
        a: 3,
        b: 4,
        c: 5,
        d: false
    },
    Test {
        a: 23,
        b: 5,
        c: 5,
        d: true
    }
);
