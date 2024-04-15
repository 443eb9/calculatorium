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

    quote::quote! {
        #[derive(Debug)]
        pub struct #phty {
            num_params: u32,
        }

        impl PhantomFunction for #phty {
            #[inline]
            fn num_params(&self) -> u32 {
                self.num_params
            }

            #[inline]
            fn solidify(&self, params: Vec<LaTexExpression>) -> LaTexElement {
                LaTexElement::#ty(<#ty>::convert(params))
            }
        }

        impl #phty {
            #[inline]
            fn new(num_params: u32) -> Self {
                Self { num_params }
            }
        }
    }
    .into()
}
