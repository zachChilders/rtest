use rtest::*;

#[derive(PartialEq, Debug)]
struct UriLookupCase {
    a: i32,
    b: String,
    c: i32,
    d: bool,
}

fn run_contrived_test(input: UriLookupCase) {
    let actual = UriLookupCase {
        a: 23,
        b: String::from("asdf"),
        c: 5,
        d: true,
    };
    assert_eq!(input, actual)
}

generate_suite!(
    run_contrived_test,
    UriLookupCase {
        a: 3,
        b: String::from("asdf"),
        c: 5,
        d: false
    },
    UriLookupCase {
        a: 23,
        b: String::from("asdf"),
        c: 5,
        d: true
    }
);
