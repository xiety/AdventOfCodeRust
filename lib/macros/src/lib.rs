use proc_macro::TokenStream;

use quote::{format_ident, quote};
use regex_lite::Regex;
use syn::parse_macro_input;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn macro_regex_test(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let fs = quote! {
        //hello
    };

    let ts = TokenStream::from(fs);

    println!("{}", ts);

    ts
}

#[proc_macro_attribute]
pub fn macro_regex(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let attr = parse_macro_input!(attr as syn::LitStr);
    let reg = Regex::new(&attr.value()).expect("Invalid regex");

    let mut parsers = Vec::new();
    let mut fields = Vec::new();

    if input.fields.len() == 0 {
        panic!("Struct must have at least one field");
    }

    if input.fields.len() != reg.captures_len() - 1 {
        panic!("Number of regex captures must equal number of struct fields")
    }

    for (i, f) in input.fields.iter().enumerate() {
        let v = format_ident!("__field_{}", i);
        let ty = &f.ty;
        let id = &f.ident;

        parsers.push(quote! {
            let #v: #ty = caps.get(#i+1).ok_or("Missing capture")?.as_str().parse()?;
        });

        fields.push(quote! {
            #id: #v,
        });
    }

    let id = &input.ident;
    let reg = format_ident!("__REGEX_{}", id.to_string().to_uppercase());

    let instance = quote! { Self {
        #(#fields)*
    } };

    let fs = quote! {

        #input

        impl ::std::str::FromStr for #id {
            type Err = Box<dyn ::std::error::Error>;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let #reg: ::regex_lite::Regex = ::regex_lite::Regex::new(#attr).unwrap();
                let caps = #reg.captures(s).ok_or("No match")?;
                #(#parsers)*
                Ok(#instance)
            }
        }
    };

    let ts = TokenStream::from(fs);

    println!("{}", ts);

    ts
}
