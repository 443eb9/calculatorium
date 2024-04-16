mod func;

#[proc_macro_derive(FromExprs)]
pub fn derive_from_exprs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    func::expand_from_exprs_derive(syn::parse(input).unwrap())
}

#[proc_macro_derive(AsPhantomFunction)]
pub fn derive_as_phantom_function(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    func::expand_as_phantom_function_derive(syn::parse(input).unwrap())
}
