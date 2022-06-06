# expr-proc-macro-def-site

Experiment with proc_macro and Span::def_site.

## Build and run

Because def_site is being used you must use RUSTFLAGS='--cfg procmacro2_semver_exempt'
to successfully build and run.

```
$ RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo run
   Compiling proc-macro2 v1.0.39
   Compiling unicode-ident v1.0.0
   Compiling quote v1.0.18
   Compiling expr-proc-macro-def-site v0.1.0 (/home/wink/prgs/rust/myrepos/expr-proc-macro-def-site)
    Finished dev [unoptimized + debuginfo] target(s) in 1.36s
     Running `target/debug/expr-proc-macro-def-site`
inner_using_outer_declarations_via_fn:+ a=10
inner_using_outer_declarations_via_fn:- a=11
inner_using_outer_declarations_via_temp:+ a=11
inner_using_outer_declarations_via_temp:- a=12
inner_using_outer_declarations_via_fn:+ a=12
inner_using_outer_declarations_via_fn:- a=13
inner_using_outer_declarations_via_temp:+ a=13
inner_using_outer_declarations_via_temp:- a=14
```

If you try to only use `call_site` for do_something_ident:
```
pub fn inner_using_outer_declarations_via_fn(_input: TokenStream) -> TokenStream {
    let do_something_ident = Ident::new("do_something", Span::call_site()); // Causes: error[E0428]: the name `do_something` is defined multiple times
    //let do_something_ident = Ident::new("do_something", Span::def_site());
    ...
```
You **don't** need to set `RUSTFLAGS` but you'll get `defined multiple times` errors:
```
$ cargo run
   Compiling expr-proc-macro-def-site v0.1.0 (/home/wink/prgs/rust/myrepos/expr-proc-macro-def-site)
error[E0428]: the name `do_something` is defined multiple times
 --> src/main.rs:6:5
  |
6 |     outer!();
  |     ^^^^^^^^
  |     |
  |     `do_something` redefined here
  |     previous definition of the value `do_something` here
  |
  = note: `do_something` must be defined only once in the value namespace of this block
  = note: this error originates in the macro `inner_using_outer_declarations_via_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0618]: expected function, found `i32`
 --> src/main.rs:6:5
  |
6 |     outer!();
  |     ^^^^^^^^
  |     |
  |     call expression requires function
  |     `do_something` has type `i32`
  |
  = note: this error originates in the macro `inner_using_outer_declarations_via_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0428, E0618.
For more information about an error, try `rustc --explain E0428`.
error: could not compile `expr-proc-macro-def-site` due to 2 previous errors
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
