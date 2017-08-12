#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Body, ConstExpr, DeriveInput, Ident, Lit, MetaItem, Path, Ty, VariantData};

#[proc_macro_derive(FromBytes, attributes(enum_type))]
pub fn derive_from_bytes(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).expect("parse_derive_input");
    let name = &ast.ident;

    let expr = match ast.body {
        Body::Struct(VariantData::Struct(ref body)) => {
            let fields: Vec<_> = body.iter()
                .filter_map(|field| field.ident.as_ref())
                .map(|ident| quote! { #ident: FromBytes::from_bytes(reader)? })
                .collect();

            quote! {
                impl FromBytes for #name {
                    // allow empty implementations, i.e. SingleMessageHeader
                    #[allow(unused_variables)]
                    fn from_bytes(reader: &mut ::std::io::Read) -> Result<Self, ::std::io::Error> {
                        Ok(#name { #(#fields),* })
                    }
                }
            }
        }
        Body::Enum(ref variants) => {
            let ty = enum_type(&ast);

            let variants: Vec<_> = variants
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    let value = match variant.discriminant {
                        Some(ConstExpr::Lit(Lit::Int(value, _))) => value,
                        _ => idx as u64,
                    };

                    let unqualified_ident = &variant.ident;
                    let ident = quote! { #name::#unqualified_ident };
                    quote! { #value => Ok(#ident) }
                })
                .collect();

            let name_str = name.as_ref();

            quote! {
                impl FromBytes for #name {
                    fn from_bytes(reader: &mut ::std::io::Read) -> Result<Self, ::std::io::Error> {
                        let value = #ty::from_bytes(reader)?;

                        match value as u64 {
                            #(#variants),*,
                            _ => Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData, format!("Unknown variant {} for {}", value, #name_str)))
                        }
                    }
                }
            }
        }
        _ => panic!("#[derive(FromBytes)] can only be used with struct or enum"),
    };

    expr.to_string().parse().expect("parse quote!")
}

#[proc_macro_derive(ToBytes, attributes(enum_type))]
pub fn derive_to_bytes(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).expect("parse_derive_input");
    let name = &ast.ident;

    let expr = match ast.body {
        Body::Struct(VariantData::Struct(ref body)) => {
            let fields: Vec<_> = body.iter()
                .filter_map(|field| field.ident.as_ref())
                .map(|ident| quote! { self.#ident.to_bytes(writer)? })
                .collect();

            quote! {
                impl ToBytes for #name {
                    // allow empty implementations, i.e. SingleMessageHeader
                    #[allow(unused_variables)]
                    fn to_bytes(&self, writer: &mut ::std::io::Write) -> Result<(), ::std::io::Error> {
                        #(#fields);*;
                        Ok(())
                    }
                }
            }
        }
        Body::Enum(ref variants) => {
            let ty = enum_type(&ast);

            let variants: Vec<_> = variants
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    let value = match variant.discriminant {
                        Some(ConstExpr::Lit(Lit::Int(value, _))) => value,
                        _ => idx as u64,
                    };

                    let unqualified_ident = &variant.ident;
                    let ident = quote! { #name::#unqualified_ident };
                    quote! { #ident => (#value as #ty).to_bytes(writer) }
                })
                .collect();

            quote! {
                impl ToBytes for #name {
                    fn to_bytes(&self, writer: &mut ::std::io::Write) -> Result<(), ::std::io::Error> {
                        match *self {
                            #(#variants),*,
                        }
                    }
                }
            }
        }
        _ => panic!("#[derive(FromBytes)] can only be used with struct or enum"),
    };

    expr.to_string().parse().expect("parse quote!")
}

#[proc_macro_derive(HasBlockLength, attributes(enum_type))]
pub fn derive_has_block_length(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).expect("parse_derive_input");
    let name = &ast.ident;

    let block_length = match ast.body {
        Body::Struct(VariantData::Struct(ref body)) => {
            let mut fields: Vec<_> = body.iter()
                .filter(|field| match field.ty {
                            // exclude Vec and Strings from block length
                            Ty::Path(None, ref path) => {
                                !path.segments
                                     .iter()
                                     .any(|seg| {
                                              let ref ident = seg.ident;
                                              ident == "Vec" || ident == "String" || ident == "Data"
                                          })
                            }
                            _ => false,
                        })
                .map(|field| {
                    let ref ty = field.ty;
                    quote! { #ty::block_length() }
                })
                .collect();

            // allow empty implementations, i.e. SingleMessageHeader
            fields.push(quote! { 0 });

            quote! {
                #(#fields)+*
            }
        }
        Body::Enum(_) => {
            let ty = enum_type(&ast);

            quote! {
                ::std::mem::size_of::<#ty>()
            }
        }
        _ => panic!("#[derive(HasBlockLength)] can only be used with structs or enums"),
    };

    let expr = quote! {
        impl HasBlockLength for #name {
            fn block_length() -> u16 {
                #block_length as u16
            }
        }
    };

    expr.to_string().parse().expect("parse quote!")
}

fn as_ty(ty: String) -> Ty {
    let ident = Ident::from(ty);
    Ty::Path(None, Path::from(ident))
}

fn enum_type(ast: &DeriveInput) -> Ty {
    named_attr(ast, "enum_type")
        .map(|value| as_ty(value))
        .unwrap_or(as_ty("u8".to_string()))
}

fn named_attr(ast: &DeriveInput, name: &str) -> Option<String> {
    ast.attrs
        .iter()
        .filter_map(|attr| match attr.value {
                        MetaItem::NameValue(ref ident, Lit::Str(ref value, _)) if ident == name => Some(value.to_owned()),
                        _ => None,
                    })
        .next()
}
