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
    //let do_something = Ident::new("do_something", Span::call_site()); // Enabling causes compile error
    let do_something = Ident::new("do_something", Span::def_site());
    quote!(
        println!("inner_using_outer_declarations_via_fn:+ a={}", a);
        fn #do_something(a: i32) -> i32 {
            a + 1
        }
        a = #do_something(a);
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

fn do_something(a: i32) -> i32 {
    a + 1
}

#[proc_macro]
pub fn outer(_input: TokenStream) -> TokenStream {
    let ds = do_something(9);
    let q = quote! {
        let mut a = #ds;
        let temp = "123";

        inner_using_outer_declarations_via_fn!();
        inner_using_outer_declarations_via_temp!();
        inner_using_outer_declarations_via_fn!();
        inner_using_outer_declarations_via_temp!();

        assert_eq!(temp, "123");
    };
    //println!("outer: q={:#?}", q);
    q.into()
}
