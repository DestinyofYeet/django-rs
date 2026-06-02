use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, GenericArgument, Ident, parse_macro_input};

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


fn prefix_ident(ident: Ident) -> Ident {
    Ident::new(&format!("macro_{ident}"), ident.span())
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

            let prefixed = prefix_ident(name);
            quote!(let mut #prefixed: Option<#ty> = None;)
        });

        let fill_options = fields.named.iter().map(|field| {
            let name = field.ident.clone().unwrap();
            let name_string = name.to_string();

            let prefixed = prefix_ident(name);
                
            quote!(
                String { .. } if matches!(Self::get_latest_column_name(#name_string), Some(col) if col == column_name) => {
                    #prefixed = column_value.from_column(column_type).ok();
                })
        });

        let check_options = fields.named.iter().map(|field| {
            let name = field.ident.clone().unwrap();
            let prefixed = prefix_ident(name.clone());
            quote!(let Some(#name) = #prefixed)
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
                fn from_iter(iter: impl Iterator<Item = django_rs::models::traits::from_iter::FromIterValue>) -> Option<Self>
                where
                    Self: Sized,
                {
                    use django_rs::models::column::{FromColumn, ToColumn};
                    #(#options)*

                    for django_rs::models::traits::from_iter::FromIterValue {
                        column_name,
                        column_value,
                        column_type,
                    } in iter {                        
                        match column_name {
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

#[proc_macro_derive(SaveData)]
pub fn derive_save_data(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(ref data) = input.data && let Fields::Named(ref fields) = data.fields {

        let name = input.ident;
        // let name_string = name.to_string();

        let save_models = fields.named.iter().map(|field| {
           let field_name = field.ident.clone().unwrap(); 
           let field_name_string = field_name.to_string();

           // let value = if is_option(&field.ty) {
           //     quote!(self.#field_name)
           // } else {
           //     quote!(self.#field_name.clone().into())
           // };

           quote!(SaveModel::new(
               Self::get_latest_column_name(#field_name_string).unwrap(),
               self.#field_name.to_column().unwrap()
           ))
        });

        return quote!(
            use django_rs::models::save::SaveModel;

            impl django_rs::models::traits::save_data::SaveData for #name {
                fn get_save_data(&self) -> Vec<SaveModel> {
                    use django_rs::models::column::ToColumn;

                    vec![
                        #(#save_models),*
                    ]
                }

            }
        ).into()
        
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive 'FromIter'",
        )
        .into_compile_error(),
    )
}
