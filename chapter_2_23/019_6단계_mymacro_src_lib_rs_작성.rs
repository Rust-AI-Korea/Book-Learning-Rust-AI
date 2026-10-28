// Chapter: chapter 2-23장. 코드 위에 그리는 마법, 디라이브(Derive) 매크로
// Section: 6단계. mymacro/src/lib.rs 작성
// Source: src/chapter_2_23.md

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(
    input: TokenStream,
) -> TokenStream {

    let input =
        parse_macro_input!(
            input as DeriveInput
        );

    let name = input.ident;

    let expanded = quote! {

        impl #name {

            pub fn hello_world() {

                println!(
                    "안녕하세요! {} 입니다.",
                    stringify!(#name)
                );
            }
        }
    };

    TokenStream::from(expanded)
}
