pub fn expand_from_exprs_derive(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let ty = input.ident;

    let syn::Data::Struct(data) = input.data else {
        panic!()
    };

    let mut fields_ctons = Vec::with_capacity(data.fields.len());
    let mut field_accessors = Vec::with_capacity(data.fields.len());

    for (field_index, field) in data.fields.iter().enumerate() {
        let field_name = field.ident.as_ref().unwrap();

        fields_ctons.push(quote::quote! {
            #field_name: std::mem::replace(&mut exprs[#field_index], LaTexExpression::default()),
        });

        field_accessors.push(quote::quote! {
            #[inline]
            pub fn #field_name(&self) -> &LaTexExpression {
                &self.#field_name
            }
        });
    }

    quote::quote! {
        impl FromExprs for #ty {
            fn convert(mut exprs: Vec<LaTexExpression>) -> Self {
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

pub fn expand_phantom_function_derive(input: syn::DeriveInput) -> proc_macro::TokenStream {
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
            fn solidify(&self, params: Vec<LaTexExpression>) -> LaTexElement {
                LaTexElement::#ty(<#ty>::convert(params))
            }
        }
    }
    .into()
}
