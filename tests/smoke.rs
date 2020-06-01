use rtest::*;

// struct TestSuite {
//     name: String,
//     test_name: String,
// }

//describe_suite!(asdf);
#[derive(GenerateTestFn, DataDriven)]
struct Test {
    field1: i32,
    field2: i32,
}

#[test]
fn run_constructed_test_function() {
    let case = Test {
        field1: 32,
        field2: 42,
    };

    let actual = Test {
        field1: 32,
        field2: 42,
    };
    case.equal(actual)
}
