use rtest::*;

#[make_answer(42)]
fn test(input: i32) -> &str {
}

#[test]
fn macro_works() {
    answer()
}

#[derive(AnswerFn)]
struct Test {
    field1: i32,
}

#[test]
fn build_test() {
    let test = Test {field1: 42};
    answer2(test);
}