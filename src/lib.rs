extern crate proc_macro;

#[allow(unused_imports)]
use proc_macro::TokenStream;
#[allow(unused_imports)]
use quote::{format_ident, quote};
#[allow(unused_imports)]
use syn::parse::{Parse, ParseStream};
#[allow(unused_imports)]
use syn::punctuated::Punctuated;
#[allow(unused_imports)]
use syn::{braced, parse_macro_input, DeriveInput, Expr, Field, Ident, Result, Token, Type};

// struct TestSuite {
//     name: String,
//     test_name: String,
// }

// impl Parse for TestSuite {
//     fn parse(input: ParseStream) -> Result<Self> {
//         //let lookahead = input.lookahead1();

//         Ok(Self {
//             name: String::from("hello"),
//             test_name: String::from("world")
//         })
//     }
// }

// #[proc_macro_attribute]
// pub fn describe(attr: TokenStream, _item: TokenStream) -> TokenStream {

//     let my_answer = attr.to_string().parse::<i32>().unwrap();
//     let answer = quote! {
//         #[test]
//         fn test_suite() {
//             assert_eq!(42, #my_answer);
//         }
//     };
//     TokenStream::from(answer)
// }

// #[proc_macro]
// pub fn test_if_it(_item: TokenStream) -> TokenStream {

//     let answer = quote! {
//         #[test]
//         fn if_test() {
//             assert_eq!(42, 42);
//         }
//     };
//     TokenStream::from(answer)
// }

// #[proc_macro]
// pub fn describe_suite(item: TokenStream) -> TokenStream {

//     eprintln!("{:?}", item);
//     let suite_name = parse_macro_input!(item as Ident);
//     // let suite_name = suite_repr.name;
//     // let test_name = suite_repr.test_name;

//     let suite = quote! {
//         #[cfg(test)]
//         mod #suite_name {
//             #[test]
//             fn test1() {
//                 assert_eq!(43, 43);
//             }
//         }
//     };
//     TokenStream::from(suite)
// }

#[proc_macro_derive(DataDriven)]
pub fn derive_data_driven(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!()
    };
    eprintln!("{:#?}", fields);

    let construct = fields.iter().map(|f|{
        let name = &f.ident;
        let actual = 42;
        quote!{ #name: #actual}
    });

    let assert = fields.iter().map(|f|{
        let name = &f.ident;
        let expected = 42;
        quote!{ assert_eq!(#expected, test.#name); }
    });


    let test_cases = quote! {
        #[test]
        fn derived_test() {

            let test = Test {
                #(#construct,)*
            };
            #(#assert)*
        }
    };
    test_cases.into()
}
