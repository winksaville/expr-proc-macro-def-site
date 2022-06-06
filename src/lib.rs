// Enabling these proc_macro:
//#![feature(proc_macro_quote, proc_macro_def_site)]
//use proc_macro::quote;
//use proc_macro::{self, Ident, Span, TokenStream};
// Causes
//   error: expected identifier, found `#`
//     --> src/main.rs:6:5
//      |
//   6  |       outer!();
//      |       ^-------
//      |       |
//      |  _____in this macro invocation
//      | |
//   7  | | }
//   ...  |
//      |
//      = note: this error originates in the macro `inner_using_outer_declarations_via_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

#[proc_macro]
pub fn inner_using_outer_declarations_via_fn(_input: TokenStream) -> TokenStream {
    //let do_something_ident = Ident::new("do_something", Span::call_site()); // Causes: error[E0428]: the name `do_something` is defined multiple times
    let do_something_ident = Ident::new("do_something", Span::def_site());
    quote!(
        println!("inner_using_outer_declarations_via_fn:+ a={}", a);
        fn #do_something_ident(a: i32) -> i32 {
            a + 1
        }
        a = #do_something_ident(a);
        println!("inner_using_outer_declarations_via_fn:- a={}", a);
    )
    .into()
}

#[proc_macro]
pub fn inner_using_outer_declarations_via_temp(_input: TokenStream) -> TokenStream {
    let temp = Ident::new("do_something", Span::call_site());
    quote!(
        println!("inner_using_outer_declarations_via_temp:+ a={}", a);
        let #temp = a + 1;
        a = #temp;
        println!("inner_using_outer_declarations_via_temp:- a={}", a);
    )
    .into()
}

#[proc_macro]
pub fn outer(_input: TokenStream) -> TokenStream {
    let q = quote! {
        let mut a = 10;
        inner_using_outer_declarations_via_fn!();
        inner_using_outer_declarations_via_temp!();
        inner_using_outer_declarations_via_fn!();
        inner_using_outer_declarations_via_temp!();
    };
    //println!("outer: q={:#?}", q);
    q.into()
}
