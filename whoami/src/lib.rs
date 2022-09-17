#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use proc_macro::TokenStream; // no need to import a specific crate for TokenStream
                             // use syn::parse;

// Generate a compile error to output struct name
#[proc_macro_derive(WhoAmI)]
pub fn whatever_you_want(tokens: TokenStream) -> TokenStream {
    // convert the input tokens into an ast, specially from a derive
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

    println!("My struct name is: <{}>", ast.ident.to_string());
    quote::quote!();

    TokenStream::new()
}

use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro,attributes(hello_macro))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    println!("compile: {}", "hello_macrobbb");

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
