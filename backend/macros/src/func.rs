use quote::quote;
use syn::{Data, Ident};

pub fn expand_from_expr_derive(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let ty = input.ident;

    let Data::Struct(data) = input.data else {
        panic!()
    };

    let num_fields = data.fields.len();
    let mut field_ctons = Vec::with_capacity(num_fields);
    let mut field_accessors = Vec::with_capacity(num_fields);

    for (field_index, field) in data.fields.iter().enumerate() {
        let field_name = field.ident.as_ref().unwrap();

        field_ctons.push(quote! {
            #field_name: expr[#field_index].take().unwrap(),
        });

        field_accessors.push(quote! {
            #[inline]
            pub fn #field_name(&self) -> &MathElement {
                &self.#field_name
            }
        });
    }

    quote! {
        impl FromExpr for #ty {
            fn convert(mut expr: Vec<Option<MathElement>>) -> Self {
                Self {
                    #(#field_ctons)*
                }
            }
        }

        impl #ty {
            #(#field_accessors)*
        }
    }
    .into()
}

pub fn expand_into_raw_expr_derive(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let ty = input.ident;

    let Data::Struct(data) = input.data else {
        panic!()
    };

    let mut field_assemblers = Vec::with_capacity(data.fields.len());
    let mut field_assembler_template = format!("\\{{}}");

    for field in &data.fields {
        let field_name = &field.ident;

        field_assemblers.push(quote! {
            self.#field_name.assemble(),
        });
        field_assembler_template.push_str("{{{}}}");
    }
    
    quote! {
        impl IntoRawExpr for #ty {
            fn assemble(&self) -> String {
                format!(
                    #field_assembler_template,
                    Self::LATEX_SYMBOL,
                    #(#field_assemblers)*
                )
            }
        }
    }
    .into()
}

pub fn expand_as_phantom_function_derive(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let ty = input.ident;
    let phty = Ident::new(&format!("Phantom{}", ty), ty.span());

    let Data::Struct(data) = input.data else {
        panic!()
    };

    let num_params = data.fields.len() as u32;

    quote! {
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

        impl IntoRawExpr for #phty {
            fn assemble(&self) -> String {
                stringify!(#ty).to_string()
            }
        }
    }
    .into()
}
