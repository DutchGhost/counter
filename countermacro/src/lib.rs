extern crate proc_macro;

use lazy_static::lazy_static;

use self::proc_macro::TokenStream;

use proc_macro_hack::proc_macro_hack;

use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Result, Token,
};

use std::sync::Mutex;
use std::path::PathBuf;
use std::collections::{HashMap, hash_map::Entry};

lazy_static! {
    static ref COUNTERS: Mutex<HashMap<PathBuf, usize>> = Mutex::new(HashMap::new());
}

struct SourceFile {
    fname: syn::LitStr,
}

impl Parse for SourceFile {
    fn parse(input: ParseStream) -> Result<Self> {
        let span: proc_macro2::Span = proc_macro2::Span::call_site();
        dbg!(span.source_file());
        let fname = input.parse().expect("STR EXPECTED");

        Ok(Self { fname })
    }
}

#[proc_macro_hack]
pub fn counter_impl(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as SourceFile);

    let b = PathBuf::from(input.fname.value());

    if let Ok(mut guard) = COUNTERS.lock() {
        let value = match guard.entry(b) {
            Entry::Vacant(v) => {
                *v.insert(1)
            }
            Entry::Occupied(mut v) => {
                *v.get_mut() += 1;
                *v.get_mut()
            }
        };

        return format!("{}", value).parse().unwrap()
    }

    panic!()
}