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
    static ref COUNTERS: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

struct Identifier {
    name: syn::LitStr,
}

impl Parse for Identifier {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse().expect("Expected Identifier");

        Ok(Self { name })
    }
}

#[proc_macro_hack]
pub fn counter_impl(tokens: TokenStream) -> TokenStream {

    let input = parse_macro_input!(tokens as Identifier);
    let name = input.name.value();

    let value = COUNTERS.lock().map(|mut guard| {
        match guard.entry(name) {
            Entry::Vacant(v) => *v.insert(0),
            Entry::Occupied(mut v) => {
                *v.get_mut() += 1;
                *v.get_mut()
            }
        }
    }).expect("Failed to update indentifiers counter.");

    format!("{}", value).parse().unwrap()
}