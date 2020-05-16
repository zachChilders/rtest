
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn make_answer(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());

    let my_answer = attr.to_string().parse::<i32>().unwrap();
    let answer = quote!{
        fn answer() {
            assert_eq!(42, #my_answer);
        }
    };
    TokenStream::from(answer)
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    println!("item: \"{}\"", item.to_string());

    let answer = quote!{
        fn answer2(input: Test) {
            assert_eq!(42, input.field1);
        }
    };
    TokenStream::from(answer)
}
