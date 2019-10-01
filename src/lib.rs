use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use countermacro::counter_impl;

#[macro_export]
macro_rules! counter {
    () => {{
        $crate::counter_impl!(file!())
    }}
}