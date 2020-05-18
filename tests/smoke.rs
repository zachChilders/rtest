use rtest::*;

// struct TestSuite {
//     name: String,
//     test_name: String,
// }

//describe_suite!(asdf);
#[derive(DataDriven)]
struct Test {
    field1: i32,
    field2: i32,
}

// #[describe(42)]
// fn run_test_struct(input: i32) -> &str {
//     derived(test);
// }
