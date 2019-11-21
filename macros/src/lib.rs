extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::iter::FromIterator;

#[proc_macro_attribute]
pub fn type_code(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let input = syn::parse_macro_input!(input as syn::ItemEnum);

    if args.len() != 1 {
        panic!("The type_code attribute expects a single integer literal argument");
    }

    let bits = &args[0];
    let name = &input.ident;
    let mut variants = Vec::new();
    let mut decodes = Vec::new();
    let mut encodes = Vec::new();

    for (index, variant) in input.variants.iter().enumerate() {
        let camel = &variant.ident;
        let camel_name = camel.to_string();

        if camel_name != "not_used" {
            let snake = syn::Ident::new(&to_snake(&camel_name), camel.span());
            let index = index as u32;

            variants.push(quote!(
                #camel(#camel<'a>),
            ));

            decodes.push(quote!(
                #index => Self::#camel(db.#snake().row(code.1)),
            ));

            encodes.push(quote!(
                Self::#camel(value) => encode(#bits, #index, value.row.index),
            ));
        }
    }

    let variants = TokenStream2::from_iter(variants);
    let decodes = TokenStream2::from_iter(decodes);
    let encodes = TokenStream2::from_iter(encodes);

    let output = quote!(
        pub enum #name<'a> {
            #variants
        }
        impl<'a> #name<'a> {
            pub(crate) fn decode(db: &'a File, code: u32) -> ParseResult<Self> {
                let code = decode(#bits, code);
                Ok(match code.0 {
                    #decodes
                    _ => return Err(ParseError::InvalidData("Invalid type code")),
                })
            }
            pub fn encode(&self) -> u32 {
                match &self {
                    #encodes
                }
            }
        }
    );

    output.into()
}

fn to_snake(camel: &str) -> String {
    let mut snake = String::new();
    for c in camel.chars() {
        if c.is_uppercase() {
            if !snake.is_empty() {
                snake.push('_');
            }
            for c in c.to_lowercase() {
                snake.push(c);
            }
        } else {
            snake.push(c);
        }
    }
    snake
}
