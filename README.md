# WD-40

Data Driven Test Framework for Rust

This crate is currently in active developement.

[Data Driven Tests](https://en.wikipedia.org/wiki/Data-driven_testing) allow for a single parametric test to generate
large numbers of test cases based on data being fed to it.  This allows a single logical test case to efficiently test and
expose edge cases in application logic.

## Usage

See [example.rs](./tests/example.rs) for detailed usage info.  This file attempts to excercise all valid code paths.

```rust
#[derive(PartialEq, Debug)]
struct TestCase {
    a: i32,
    b: String,
    c: i32,
    d: bool,
    should_fail: bool // should_fail is not required, but recommended.
}

// Any panics in here will cause a test failure.  Unwrap it all!
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

```

## Future Work

- [ ] Improved error propagation
- [ ] `trybuild` tests
- [ ] Combinatric Test Case Construction
- [ ] Fluid Syntax
