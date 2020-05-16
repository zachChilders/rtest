use rtest::*;

describe_suite!(describe_suite);
test_if_it!("test1");
#[derive(DataDriven)]
struct Test {
    field1: i32,
}

#[describe(42)]
fn run_test_struct(input: i32) -> &str {
    derived(test);
}

