extern crate alloc;
extern crate core;


use proc_macro::TokenStream;

use syn::__private::TokenStream2;

use crate::volatile_address::expand_volatile_address;
use crate::volatile_bits::force_expand_volatile_bits;

mod volatile_bits;
mod volatile_address;


#[proc_macro_attribute]
pub fn volatile_address(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    join(
        join_address_derives(item.clone()),
        expand_volatile_address(item),
    )
        .into()
}


#[proc_macro_attribute]
pub fn volatile_bits(attributes: TokenStream, input: TokenStream) -> TokenStream {
    join(
        proc_macro2::TokenStream::from(input.clone()),
        force_expand_volatile_bits(input, attributes),
    ).into()
}


fn join_address_derives(item: TokenStream) -> TokenStream2 {
    join(
        quote::quote! {
            #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
            #[repr(transparent)]
        },
        TokenStream2::from(item),
    )
}


fn join(lhs: TokenStream2, rhs: TokenStream2) -> TokenStream2 {
    let union = quote::quote! {
        #lhs
        #rhs
    };
    union
}