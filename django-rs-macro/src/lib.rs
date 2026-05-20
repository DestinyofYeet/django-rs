use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, GenericArgument, parse_macro_input};

use syn::{Type, TypePath, PathArguments};

fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let segment = path.segments.last();

            matches!(segment, Some(seg) if seg.ident == "Option")
        }
        _ => false,
    }
}

fn option_inner_type(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    let segment = type_path.path.segments.last()?;

    if segment.ident != "Option" {
        return None;
    }

    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };

    let GenericArgument::Type(inner_ty) = args.args.first()? else {
        return None;
    };

    Some(inner_ty)
}

#[proc_macro_derive(FromIter)]
pub fn derive_from_iter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data
        && let Fields::Named(ref fields) = data.fields
    {
        let options = fields.named.iter().map(|field| {
            let ty = field.ty.clone();
            let name = field.ident.clone().unwrap();
            let ty = if is_option(&ty) { option_inner_type(&ty).unwrap().clone() } else { ty };
            quote!(let mut #name: Option<#ty> = None;)
        });

        let fill_options = fields.named.iter().map(|field| {
            let name = field.ident.clone().unwrap();
            let name_string = name.to_string();
                
            quote!(
                String { .. } if matches!(Self::get_latest_column_name(#name_string), Some(col) if col == key) => {
                    #name = value.parse().ok();    
                })
        });

        let check_options = fields.named.iter().map(|field| {
            let name = field.ident.clone().unwrap();
            quote!(let Some(#name) = #name)
        });

        let construct_self = fields.named.iter().map(|field| {
            let name = field.ident.clone().unwrap();

            let value = if is_option(&field.ty) { quote!(Some(#name)) } else { quote!(#name) };
            
            quote!(
                #name: #value
            )
        });

        let name = input.ident;

        return quote!(
            impl django_rs::models::traits::from_iter::FromIter for #name {
                fn from_iter(iter: impl Iterator<Item = (String, String)>) -> Option<Self>
                where
                    Self: Sized,
                {
                    #(#options)*

                    for (key, value) in iter {
                        match value {
                            #(#fill_options)*
                            
                            _ => {}
                        }
                    }

                    if #(#check_options)&&* {
                        return Some(Self {
                            #(#construct_self),*
                        });
                    }

                    None
                }

            }
        )
        .into();
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive 'FromIter'",
        )
        .into_compile_error(),
    )
}
