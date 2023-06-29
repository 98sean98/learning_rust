use proc_macro::TokenStream;
use quote::quote;
use syn;

// don't need to `use hello_macro::Hello_Macro`

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // turns TokenStream into DeriveInput

    // Build the trait implementation
    impl_hello_macro(&ast)
}
// the `hello_macro_derive` function is more or less the same for any procedural macro derive
// only the `impl_hello_macro` would be different
// `hello_macro_derive` is called everytime `#[derive(HelloMacro)]`


fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // `quote!` lets us define the rust code to return
    // provides templating mechanism for `#name`

    gen.into() // `.into()` turns into TokenStream
}
