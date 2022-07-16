use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Nothing, Parser},
    parse_macro_input, Fields, ItemStruct,
};

/// Attribute macro that adds `pub version: u8` and
/// `pub flags: u32` fields.
#[proc_macro_attribute]
pub fn full_box(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as Nothing);
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub version: u8 })
                .unwrap(),
        );

        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub flags: u32 })
                .unwrap(),
        );
    }

    quote! { #item_struct }.into()
}
