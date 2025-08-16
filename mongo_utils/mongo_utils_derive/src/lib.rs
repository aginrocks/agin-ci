use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{Data, Fields};

#[derive(FromField)]
#[darling(attributes(serde))]
#[allow(dead_code)]
struct SerdeField {
    ident: Option<syn::Ident>,
    rename: Option<String>,

    alias: Option<String>,
    default: Option<String>,
    skip_serializing_if: Option<String>,
    serialize_with: Option<String>,
    deserialize_with: Option<String>,
    with: Option<String>,
    bound: Option<String>,
    getter: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(mongo_utils))]
struct AnnotatedStruct {
    // ident: syn::Ident,
    collection: String,
    // path_prefix: Option<String>,
    // data: darling::ast::Data<(), AnnotatedField>,
}

#[proc_macro_error]
#[proc_macro_derive(JoinPipeline, attributes(mongo_utils))]
pub fn join_pipeline_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_join_pipeline(&ast)
}

fn impl_join_pipeline(ast: &syn::DeriveInput) -> TokenStream {
    // TODO: Make error handling better
    let annotated = AnnotatedStruct::from_derive_input(ast).unwrap_or_else(|_| {
        abort!(
            ast,
            "JoinPipeline requires a `#[mongo_utils(collection = \"\")]` attribute applied to the struct"
        )
    });

    let collection = annotated.collection;

    let name = &ast.ident;

    let fields = {
        match &ast.data {
            Data::Struct(data_struct) => match &data_struct.fields {
                Fields::Named(fields) => &fields.named,
                _ => abort!(ast, "JoinPipeline only works on structs with named fields"),
            },
            _ => abort!(ast, "JoinPipeline can only be applied on structs"),
        }
    };

    let field_names = fields
        .iter()
        .map(|f| {
            if let Ok(serde_attrs) = SerdeField::from_field(f)
                && let Some(rename) = serde_attrs.rename
            {
                return rename;
            }

            f.ident
                .clone()
                .unwrap_or_else(|| {
                    abort!(ast, "Unsupported field");
                })
                .to_string()
        })
        .collect::<Vec<_>>();

    let generated = quote! {
        impl ::mongo_utils::JoinPipelineBuilder for #name {
            fn join_pipeline(local_field: &str, foreign_field: &str) -> Vec<::mongo_utils::Document> {
                vec![
                    ::mongo_utils::doc! {
                        "$lookup": {
                            "from": #collection,
                            "localField": local_field,
                            "foreignField": foreign_field,
                            "as": local_field,
                        }
                    },
                    ::mongo_utils::doc! {
                        "$unwind": {
                            "path": format!("${local_field}"),
                            "preserveNullAndEmptyArrays": true,
                        }
                    },
                    ::mongo_utils::doc! {
                        "$addFields": {
                            local_field: {
                                "$cond": {
                                    "if": { "$ne": [format!("${local_field}"), null] },
                                    "then": {
                                        #( #field_names: format!("${local_field}.{}", #field_names), )*
                                    },
                                    "else": null
                                }
                            }
                        }
                    }
                ]
            }
        }
    };
    generated.into()
}
