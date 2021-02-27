//! This is a derive procedural macro that will let you add custom derive
//! and attributes over structs, enums and unions. This derive will add two impl on the
//! type. The `as_string()` method returns a json serialized string representation of the type
//! with any meta information annotated with `structype_meta("key"=val)` attribute,
//! while the `print_fields()` method will print the same to STDOUT.
//! This macro will panic at compile time if annotated over tuple and unit structs.
//!
//! # Example:
//! ```
//! use structype_derive::StrucType;
//! #[derive(StrucType)]
//! // #[structype_meta("labelover_ride=name")] This will panic the macro
//! struct UserStruct {
//!     #[structype_meta(override_name="Primary ID", order="1")]
//!     id: i64,
//!     #[structype_meta(override_name="name", order="0")]
//!     username: String,
//!     org: String,
//!     details: Details,
//! }
//!
//! #[derive(StrucType)]
//! struct Details {
//!     user_attributes: std::collections::HashMap<String, String>,
//! }
//!
//! fn print_struct_fields() {
//!     UserStruct::print_fields();
//!     let data = UserStruct::as_string();
//!     println!("{}", data);
//!     Details::print_fields();
//!     let data = Details::as_string();
//!     println!("{}", data);
//! }
//! ```
//! The above will generate and return a json serialized string representation where the key is
//! the struct's field name and the value is a `HashMap<String, String>` of `structype_meta`'s key-val. If the `structype_meta` is
//! absent, the field's associated value would be an empty `{}`.
//!
//! # Output:
//! ```json
//!  [
//!      {
//!          "id": {
//!              "override_name": "Primary ID",
//!              "order": "1"
//!          }
//!      },
//!      {
//!          "username": {
//!              "override_name": "name",
//!              "order": "0"
//!          }
//!      },
//!      {
//!          "org": {}
//!      },
//!      {
//!          "details": {}
//!      }
//!  ]
//! ```

use proc_macro::{self, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed};
use serde::ser::{Serialize, SerializeMap, Serializer};

type StrucTypeMap = Vec<TypeMap>;

#[derive(Debug, Clone)]
struct TypeMap {
    field_name: String,
    meta: HashMap<String, String>,
}

impl Serialize for TypeMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_map(Some(self.meta.len() + 1))?;
        seq.serialize_entry(&self.field_name, &self.meta)?;

        seq.end()
    }
}

#[proc_macro_derive(StrucType, attributes(structype_meta))]
pub fn structmap(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let name = &ast.ident;
    let top_attr = &ast.attrs;
    for attr in top_attr.iter() {
        let meta = attr.parse_meta();
        match meta {
            _ => panic!("Cannot apply attribute outside a type. Applicable only inside the type on type fields."),
        }
    }

    let description = match &ast.data {
        syn::Data::Struct(s) => {
            match &s.fields {
                syn::Fields::Named(FieldsNamed { named, .. }) => {
                    let mut structype_map: StrucTypeMap = Vec::new();
                    let iters = named.iter().map(|f| (&f.ident, &f.attrs));
                    for (if_ident, attrs) in iters {
                        if let Some(ident) = if_ident {
                            if attrs.len() > 0 {
                                let mut record = TypeMap {
                                    field_name: ident.to_string(),
                                    meta: HashMap::new(),
                                };
                                for attr in attrs.iter() {
                                    let meta = attr.parse_meta().unwrap();
                                    match meta {
                                        syn::Meta::List(metalist) => {
                                            let pairs = metalist
                                                .nested
                                                .into_pairs()
                                                .map(|pair| pair.into_value());
                                            for pair in pairs {
                                                match pair {
                                                syn::NestedMeta::Meta(meta) => match meta {
                                                    syn::Meta::Path(_) => {panic!(r#"invalid. Use the format structype_meta(label="foo", ord="10")"#)}
                                                    syn::Meta::List(_) => {panic!(r#"invalid. Use the format structype_meta(label="foo", ord="10")"#)}
                                                    syn::Meta::NameValue(meta_nameval) => {
                                                        let path = meta_nameval.path;
                                                        match meta_nameval.lit {
                                                            syn::Lit::Str(str_lit) => {

                                                                record.meta.insert(path.get_ident().unwrap().to_string(), str_lit.value());
                                                            }
                                                            _ => {panic!("Only string type is supported now")}
                                                        }
                                                    }
                                                }
                                                syn::NestedMeta::Lit(_) => {panic!("Lit is not applicable. Annotate as key-value")}
                                            }
                                            }
                                            structype_map.push(record.clone());
                                        }

                                        _ => panic!(
                                            r#"Not applicable. Present a list of key-value attributes like structype_meta(label="foo", ord="10")"#
                                        ),
                                        // syn::Meta::Path(_) => {}
                                    }
                                }
                            } else {
                                let val = TypeMap {
                                    field_name: ident.to_string(),
                                    meta: HashMap::new(),
                                };
                                structype_map.push(val);
                            }
                        }
                    }
                    serde_json::to_string(&structype_map).unwrap()
                }
                syn::Fields::Unnamed(_) => panic!("Not applicable to Tuple structs"),

                syn::Fields::Unit => panic!("Not applicable to Unit structs"),
            }
        }
        // Enums parsing starts here
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let mut structype_map: StrucTypeMap = Vec::new();
            let iters = variants.iter().map(|f| (&f.ident, &f.attrs));
            for (if_ident, attrs) in iters {
                if attrs.len() > 0 {
                    let mut record = TypeMap {
                        field_name: if_ident.to_string(),
                        meta: HashMap::new(),
                    };
                    for attr in attrs.iter() {
                        let meta = attr.parse_meta().unwrap();
                        match meta {
                            syn::Meta::List(metalist) => {
                                let pairs =
                                    metalist.nested.into_pairs().map(|pair| pair.into_value());
                                for pair in pairs {
                                    match pair {
                                        syn::NestedMeta::Meta(meta) => match meta {
                                            syn::Meta::Path(_) => {
                                                panic!(r#"invalid. Add as key="value#""#)
                                            }
                                            syn::Meta::List(_) => {
                                                panic!(r#"invalid. Add as key="value#""#)
                                            }
                                            syn::Meta::NameValue(meta_nameval) => {
                                                let path = meta_nameval.path;
                                                match meta_nameval.lit {
                                                    syn::Lit::Str(str_lit) => {
                                                        record.meta.insert(
                                                            path.get_ident().unwrap().to_string(),
                                                            str_lit.value(),
                                                        );
                                                    }
                                                    _ => {
                                                        panic!("Only string type is supported now")
                                                    }
                                                }
                                            }
                                        },
                                        syn::NestedMeta::Lit(_) => {
                                            panic!("Lit is not applicable. Annotate as key-value")
                                        }
                                    }
                                }
                                structype_map.push(record.clone());
                            }

                            _ => panic!(
                                r#"Not applicable. Present a list of key-value attributes like structype_meta(label="foo", ord="10")"#
                            ),
                            // syn::Meta::Path(_) => {}
                        }
                    }
                } else {
                    let val = TypeMap {
                        field_name: if_ident.to_string(),
                        meta: HashMap::new(),
                    };
                    structype_map.push(val);
                }
            }
            serde_json::to_string(&structype_map).unwrap()
        }
        syn::Data::Union(DataUnion {
            fields: FieldsNamed { named, .. },
            ..
        }) => {
            let mut structype_map: StrucTypeMap = Vec::new();
            let iters = named.iter().map(|f| (&f.ident, &f.attrs));
            for (if_ident, attrs) in iters {
                if let Some(ident) = if_ident {
                    if attrs.len() > 0 {
                        let mut record = TypeMap {
                            field_name: ident.to_string(),
                            meta: HashMap::new(),
                        };
                        for attr in attrs.iter() {
                            let meta = attr.parse_meta().unwrap();
                            match meta {
                                syn::Meta::List(metalist) => {
                                    let pairs =
                                        metalist.nested.into_pairs().map(|pair| pair.into_value());
                                    for pair in pairs {
                                        match pair {
                                            syn::NestedMeta::Meta(meta) => match meta {
                                                syn::Meta::Path(_) => {
                                                    panic!(r#"invalid. Use the format structype_meta(label="foo", ord="10")"#)
                                                }
                                                syn::Meta::List(_) => {
                                                    panic!(r#"invalid. Use the format structype_meta(label="foo", ord="10")"#)
                                                }
                                                syn::Meta::NameValue(meta_nameval) => {
                                                    let path = meta_nameval.path;
                                                    match meta_nameval.lit {
                                                        syn::Lit::Str(str_lit) => {
                                                            record.meta.insert(
                                                                path.get_ident()
                                                                    .unwrap()
                                                                    .to_string(),
                                                                str_lit.value(),
                                                            );
                                                        }
                                                        _ => panic!(
                                                            "Only string type is supported now"
                                                        ),
                                                    }
                                                }
                                            },
                                            syn::NestedMeta::Lit(_) => panic!(
                                                r#"Literal is not applicable. Annotate as key-value like structype_meta(label="foo#", ord="10")"#
                                            ),
                                        }
                                    }
                                    structype_map.push(record.clone());
                                }

                                _ => panic!(
                                    r#"Not applicable. Present a list of key-value attributes like structype_meta(label="foo", ord="10")"#
                                ),
                            }
                        }
                    } else {
                        let val = TypeMap {
                            field_name: ident.to_string(),
                            meta: HashMap::new(),
                        };
                        structype_map.push(val);
                    }
                }
            }
            serde_json::to_string(&structype_map).unwrap()
        }
    };

    let output = quote! {
    impl #name {
        pub fn print_fields() {
        println!("{}", #description);
        }

        pub fn as_string() -> String {
            return #description.to_string()
        }
    }
    };

    output.into()
}
