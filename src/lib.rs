extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, Ident, Expr, Result, Token};

#[derive(Debug)]
struct SuiteInput {
    test_name: Ident,
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
    data: Expr,
}

impl Parse for SuiteInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let test_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let tests = input.parse_terminated(Test::parse)?;

        Ok(SuiteInput {
            test_name,
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
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let data = input.parse()?;

        Ok(Field {
            name,
            data,
        })
    }
}

#[proc_macro]
pub fn generate_suite(item: TokenStream) -> TokenStream {
    let suite = parse_macro_input!(item as SuiteInput);
    let suite_name = suite.test_name.clone();
    //println!("Suite: {:?}", suite.tests);

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