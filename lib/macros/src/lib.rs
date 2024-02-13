use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::parse_macro_input;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn macro_regex(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let attr = parse_macro_input!(attr as syn::LitStr);

    let mut parsers = Vec::new();
    let mut fields = Vec::new();

    for f in input.fields.iter() {
        let ty = &f.ty;
        let id = &f.ident;
        let name = &f.ident.as_ref().unwrap().to_string();
        let v = format_ident!("__{}", name);

        parsers.push(quote! {
            let #v: #ty = caps.name(#name).expect(#name).as_str().parse().unwrap();
        });

        fields.push(quote! {
            #id: #v,
        });
    }

    let id = &input.ident;

    let fs = quote! {

        #input

        impl ::std::str::FromStr for #id {
            type Err = Box<dyn ::std::error::Error>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let reg: ::regex_lite::Regex = ::regex_lite::Regex::new(#attr).unwrap();
                let caps = reg.captures(s).unwrap();

                #(#parsers)*

                Ok(Self {
                    #(#fields)*
                })
            }
        }
    };

    let ts = TokenStream::from(fs);

    ts
}
