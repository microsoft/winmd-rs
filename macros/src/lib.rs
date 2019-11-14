extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use syn::*;
use proc_macro2::*;
use std::iter::FromIterator;

#[proc_macro_attribute]
pub fn type_encoding(args: TokenStream, input: TokenStream) -> TokenStream {
    // println!("args: {}", args.to_string());
    // println!("input: {}", input.to_string());

    let args = syn::parse_macro_input!(args as syn::AttributeArgs);

    if args.len() != 1 {
        panic!("type_encoding macro expects a single integer literal argument");
    }

    let type_code = &args[0];
    let input = syn::parse_macro_input!(input as syn::ItemEnum);
    let name = &input.ident;
    let variants = input.variants.iter();

    //let mut output: Vec<TokenStream> = Vec::new();
    // // let type_index = match &args[0] {
    // //     NestedMeta::Lit(value) => match value {
    // //         Lit::Int(value) => value,
    // //         _ => panic!("type_encoding macro requires a single integer argument"),
    // //     },
    // //     _ => panic!("type_encoding macro requires a single integer argument"),
    // // };

    // for variant in &input.variants
    // {
    //     if variant.ident != "Unused"
    //     {
    //         println!("{}", variant.ident.to_string());
    //     }

    //     index += 1;
    // }

    // output.push(input.to_token_stream().into());

    // TokenStream::from_iter(output)

    let output = quote!(
        pub enum #name<'a> {
            #(#variants),*
        }
    );

    println!("{}", output);

    quote!(
    ).into()
}
