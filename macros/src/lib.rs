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
    let mut enumerator = 0;

    for variant in input.variants.iter() {
        let camel = &variant.ident;
        let camel_name = camel.to_string();

        if let Some((_, value)) = &variant.discriminant {
            if let syn::Expr::Lit(value) = value {
                if let syn::Lit::Int(value) = &value.lit {
                    enumerator = value.base10_parse::<u32>().unwrap();
                }
            }
        }

        let snake = syn::Ident::new(&to_snake(&camel_name), camel.span());

        variants.push(quote!(
            #camel(#camel<'a>),
        ));

        decodes.push(quote!(
            #enumerator => Self::#camel(file.#snake().row(code.1)),
        ));

        encodes.push(quote!(
            Self::#camel(value) => ((value.row.index + 1) << #bits) | #enumerator,
        ));

        enumerator += 1;
    }

    let variants = TokenStream2::from_iter(variants);
    let decodes = TokenStream2::from_iter(decodes);
    let encodes = TokenStream2::from_iter(encodes);

    let output = quote!(
        pub enum #name<'a> {
            #variants
        }
        impl<'a> #name<'a> {
            pub(crate) fn decode(file: &'a File, code: u32) -> ParseResult<Self> {
                let code = (code & ((1 << #bits) - 1), (code >> #bits) - 1);
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
