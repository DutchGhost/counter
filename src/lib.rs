use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use countermacro::counter_impl as counter;
