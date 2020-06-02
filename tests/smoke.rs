use rtest::*;


#[derive(GenerateTestFn)]
struct Test {
    field1: i32,
    field2: i32,
}

fn run_my_test() {
    println!("Hello World!");
}

describe_suite!(
    Test {
        a: 3,
    },
    Test {
        b: 4,
        c: 5,
    }
);

#[test]
fn run_constructed_test_function() {
    let expected = Test {
        field1: 32,
        field2: 42,
    };

    let actual = Test {
        field1: 32,
        field2: 42,
    };
    expected.equal(&actual);
    actual.equal(&expected);
}
