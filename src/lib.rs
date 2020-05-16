
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident};

#[proc_macro_attribute]
pub fn describe(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());

    let my_answer = attr.to_string().parse::<i32>().unwrap();
    let answer = quote!{
        #[test]
        fn test_suite() {
            assert_eq!(42, #my_answer);
        }
    };
    TokenStream::from(answer)
}

#[proc_macro]
pub fn test_if_it(item:TokenStream) -> TokenStream {
    println!("item: \"{}\"", item.to_string());

    let answer = quote!{
        #[test]
        fn if_test() {
            assert_eq!(42, 42);
        }
    };
    TokenStream::from(answer)
}


#[proc_macro]
pub fn describe_suite(item: TokenStream) -> TokenStream {
    println!("describe item: \"{}\"", item.to_string());
   let suite_name = parse_macro_input!(item as Ident);
    let suite = quote!{
        #[cfg(test)]
        mod #suite_name {
            #[test]
            fn test_name1() {
                assert_eq!(43, 43);
            }
        }
    };
    TokenStream::from(suite)
}

#[proc_macro_derive(DataDriven)]
pub fn derive_data_driven(item: TokenStream) -> TokenStream {
    println!("item: \"{}\"", item.to_string());

    let answer = quote!{
        #[test]
        fn derived_test() {
            let test = Test {field1: 42};
            assert_eq!(42, test.field1);
        }
    };
    TokenStream::from(answer)
}
