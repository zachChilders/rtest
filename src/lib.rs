extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, DeriveInput, Ident, Lit, Result, Token};

#[derive(Debug)]
struct SuiteInput {
    test_name: Ident,
    comma: Token![,],
    tests: Punctuated<Test, Token![,]>,
}

#[derive(Debug)]
struct Test {
    ident: Ident,
    brace_token: syn::token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

#[derive(Clone, Debug)]
struct Field {
    name: Ident,
    colon_token: Token![:],
    data: Lit,
}

impl Parse for SuiteInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let test_name = input.parse()?;
        let comma = input.parse()?;
        let tests = input.parse_terminated(Test::parse)?;
        Ok(SuiteInput {
            test_name,
            comma,
            tests,
        })
    }
}

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        let ident = input.parse()?;
        let brace_token = braced!(content in input);
        let fields = content.parse_terminated(Field::parse)?;

        Ok(Test {
            ident,
            brace_token,
            fields,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Field {
            name: input.parse()?,
            colon_token: input.parse()?,
            data: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn describe_suite(item: TokenStream) -> TokenStream {
    let suite = parse_macro_input!(item as SuiteInput);
    let suite_name = suite.test_name.clone();
    println!("Suite: {:?}", suite.tests);

    let tests = suite.tests.iter().enumerate().map(|(i, f)| {
        let name = f.ident.clone();
        let test_name = format_ident!("{}_{}", name.to_string().to_lowercase(), i.to_string());
        let fields = f.fields.iter().map(|f| {
            let name = f.name.clone();
            let value = f.data.clone();
            quote! {
                #name: #value,
            }
        });
        quote! {
            #[test]
            fn #test_name() {
                crate::#suite_name(crate::#name{
                    #(#fields)*
                })
            }
        }
    });

    let suite = quote! {
        #[cfg(test)]
        mod #suite_name {
            #(#tests)*
        }
    };
    suite.into()
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
        impl PartialEq for Test {
            fn eq(&self, other: &Self) -> bool {
                #(#assert)*
                true
            }
        }
    };
    test_fn.into()
}
