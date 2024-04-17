mod func;

#[proc_macro_derive(FromExpr)]
pub fn derive_from_expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    func::expand_from_expr_derive(syn::parse(input).unwrap())
}

#[proc_macro_derive(IntoRawExpr)]
pub fn derive_into_raw_expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    func::expand_into_raw_expr_derive(syn::parse(input).unwrap())
}

#[proc_macro_derive(AsPhantomFunction)]
pub fn derive_as_phantom_function(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    func::expand_as_phantom_function_derive(syn::parse(input).unwrap())
}

#[proc_macro_derive(AsPhantomOperator, attributes(priority))]
pub fn derive_as_phantom_operator(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    func::expand_as_phantom_operator_derive(syn::parse(input).unwrap())
}
