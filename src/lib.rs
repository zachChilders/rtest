extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, DeriveInput, Ident, Result, Token, Type, Lit};

#[derive(Debug)]
struct SuiteInput {
    name_of_test: Ident,
    tests: Punctuated<Test, Token![,]>,
}

#[derive(Debug)]
struct Test {
    ident: Ident,
    brace_token: syn::token::Brace,
    fields: Punctuated<Fields, Token![,]>,

}

#[derive(Debug)]
struct Fields {
    name: Ident,
    colon_token: Token![:],
    data: Lit,
}

impl Parse for SuiteInput {
    fn parse(input: ParseStream) -> Result<Self> {
       // let test_function = input.parse_terminated(Ident::parse)?;
        let tests = input.parse_terminated(Test::parse)?;  
        Ok(SuiteInput {
            tests,
        })
    }
}

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        let ident = input.parse()?;
        let brace_token = braced!(content in input);
        let fields = content.parse_terminated(Fields::parse)?;  

        Ok(Test {
            ident,
            brace_token,
            fields,
        })
    }
}

impl Parse for Fields {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Fields {
            name: input.parse()?,
            colon_token: input.parse()?,
            data: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn describe_suite(item: TokenStream) -> TokenStream {
    let suite_name = parse_macro_input!(item as SuiteInput);
    println!("Suite: {:?}", suite_name);

    // let suite = quote! {
    //     #[cfg(test)]
    //     mod #suite_name {
    //         #[test]
    //         fn test_name1() {
    //             assert_eq!(43, 43);
    //         }
    //     }
    // };
    // suite.into()
    TokenStream::new()
}

#[proc_macro_derive(GenerateTestFn)]
pub fn derive_data_driven_test(item: TokenStream) -> TokenStream {
    println!("Raw input: {:?}", item.clone());
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

    let assert = fields.iter().map(|f| {
        let name = &f.ident;
        let name_string = &f.ident.clone().unwrap().to_string();
        quote! {
        assert_eq!(self.#name, expected.#name,
            "{name}: {actual} is not {expected}",
            name=#name_string,
            actual=self.#name,
            expected=expected.#name); }
    });

    let test_fn = quote! {
        impl Test {
            fn equal(&self, expected: &Test) {
                #(#assert)*
            }
        }
    };
    test_fn.into()
}
