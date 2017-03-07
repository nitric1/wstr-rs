#[macro_use]
extern crate proc_macro_hack;
#[macro_use]
extern crate wstr_impl;

pub use wstr_impl::*;

proc_macro_expr_decl!(wstr! => wstr_impl);

#[cfg(feature = "widestring")]
#[macro_export]
macro_rules! widestr {
    ($str:tt) => (widestring::WideStr::from_slice(wstr!($str)))
}
