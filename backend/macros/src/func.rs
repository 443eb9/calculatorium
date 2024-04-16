pub fn expand_from_expr_derive(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let ty = input.ident;

    let syn::Data::Struct(data) = input.data else {
        panic!()
    };

    let mut fields_ctons = Vec::with_capacity(data.fields.len());
    let mut field_accessors = Vec::with_capacity(data.fields.len());

    for (field_index, field) in data.fields.iter().enumerate() {
        let field_name = field.ident.as_ref().unwrap();

        fields_ctons.push(quote::quote! {
            #field_name: expr[#field_index].take().unwrap(),
        });

        field_accessors.push(quote::quote! {
            #[inline]
            pub fn #field_name(&self) -> &MathElement {
                &self.#field_name
            }
        });
    }

    quote::quote! {
        impl FromExpr for #ty {
            fn convert(mut expr: Vec<Option<MathElement>>) -> Self {
                Self {
                    #(#fields_ctons)*
                }
            }
        }

        impl #ty {
            #(#field_accessors)*
        }
    }
    .into()
}

pub fn expand_as_phantom_function_derive(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let ty = input.ident;
    let phty = syn::Ident::new(&format!("Phantom{}", ty), ty.span());

    let syn::Data::Struct(data) = input.data else {
        panic!()
    };

    let num_params = data.fields.len() as u32;

    quote::quote! {
        #[derive(Debug, Default)]
        pub struct #phty;

        impl PhantomFunction for #phty {
            #[inline]
            fn num_params(&self) -> u32 {
                #num_params
            }

            #[inline]
            fn solidify(&self, params: Vec<Option<MathElement>>) -> MathFunction {
                MathFunction::#ty(Box::new(<#ty>::convert(params)))
            }
        }
    }
    .into()
}
