//! This is a derive procedural macro that will let you add custom derive
//! and attributes over structs and enums. This derive will add two impl on the
//! type. `as_string()` returns a json serialized string key-value representation
//! where key is the type's field name and value is the attribute set in the
//! `structype_label`, while`print_fields()` function will print the same to STDOUT.
//! This macro cannot be applied over tuple structs and unit structs, so it will panic if these
//! types are annotated with this derive and won't let you compile.
//!
//! # Example:
//! ```
//! use structype_derive::StrucType;
//!
//! #[derive(StrucType)]
//! // #[structype_label = "over_ride name"] // This will panic the macro
//! struct MyStruct {
//!     #[structype_label = "Overridde name for string"]
//!     _my_string: String,
//!     #[structype_label = "int_override"]
//!     _my_int64: i64,
//!     _my_float: f64,
//!     _my_nested_struct: MyNestedStruct,
//! }
//!
//! #[derive(StrucType)]
//! struct MyNestedStruct {
//!     _my_nested_struct_string: String,
//! }
//!
//! fn print_struct_fields() {
//!     MyStruct::print_fields();
//!     let data = MyStruct::as_string();
//!     println!("{}", data);
//!     MyNestedStruct::print_fields();
//!     let data = MyNestedStruct::as_string();
//!     println!("{}", data);
//! }
//! ```
//! The above will generate and return a json serialized string representation where the key is
//! the struct's field name and the value is the `structype_label`'s value. If the `structype_label` is
//! absent, the value will be the same as that of the key.
//!
//! # Output:
//! ```json
//! {
//!	    "_my_string": "Overridde name for string",
//!	    "_my_int64": "int_override",
//!	    "_my_float": "_my_float",
//!	    "_my_nested_struct": "_my_nested_struct"
//! }
//! ```

use proc_macro::{self, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed};

use serde::ser::{Serialize, SerializeMap, Serializer};

struct TypeMap {
    map: HashMap<String, String>,
}

impl Serialize for TypeMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_map(Some(self.map.len()))?;
        for (k, v) in &self.map {
            seq.serialize_entry(&k.to_string(), &v)?;
        }
        seq.end()
    }
}

#[proc_macro_derive(StrucType, attributes(structype_label))]
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
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let mut typemap = TypeMap {
                    map: HashMap::new(),
                };
                let iters = named.iter().map(|f| (&f.ident, &f.attrs));
                for (if_ident, attrs) in iters {
                    if let Some(ident) = if_ident {
                        if attrs.len() == 1 {
                            for attr in attrs.iter() {
                                let meta = attr.parse_meta().unwrap();
                                match meta {
                                    syn::Meta::NameValue(meta_nameval) => match meta_nameval.lit {
                                        syn::Lit::Str(string_literal) => {
                                            typemap
                                                .map
                                                .insert(ident.to_string(), string_literal.value());
                                        }
                                        _ => panic!("Only string value is supported"),
                                    },
                                    _ => panic!("Path and List is not applicable"),
                                }
                            }
                        } else {
                            typemap.map.insert(ident.to_string(), ident.to_string());
                        }
                    }
                }
                serde_json::to_string(&typemap.map).unwrap()
            }
            syn::Fields::Unnamed(_) => panic!("Tuple structs not applicable"),

            syn::Fields::Unit => panic!("Not applicable to Unit structs"),
        },
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let mut typemap = TypeMap {
                map: HashMap::new(),
            };
            let iters = variants.iter().map(|f| (&f.ident, &f.attrs));
            for (if_ident, attrs) in iters {
                if attrs.len() == 1 {
                    for attr in attrs.iter() {
                        let meta = attr.parse_meta().unwrap();
                        match meta {
                            syn::Meta::NameValue(meta_nameval) => match meta_nameval.lit {
                                syn::Lit::Str(string_literal) => {
                                    typemap
                                        .map
                                        .insert(ast.ident.to_string(), string_literal.value());
                                }
                                _ => panic!("Only string value is supported"),
                            },
                            _ => panic!("Path and List is not applicable"),
                        }
                    }
                } else {
                    typemap
                        .map
                        .insert(if_ident.to_string(), ast.ident.to_string());
                }
            }
            serde_json::to_string(&typemap.map).unwrap()
        }
        syn::Data::Union(DataUnion {
            fields: FieldsNamed { named, .. },
            ..
        }) => {
            let mut typemap = TypeMap {
                map: HashMap::new(),
            };
            let iters = named.iter().map(|f| (&f.ident, &f.attrs));
            for (if_ident, attrs) in iters {
                if let Some(ident) = if_ident {
                    if attrs.len() == 1 {
                        for attr in attrs.iter() {
                            let meta = attr.parse_meta().unwrap();
                            match meta {
                                syn::Meta::NameValue(meta_nameval) => match meta_nameval.lit {
                                    syn::Lit::Str(string_literal) => {
                                        typemap
                                            .map
                                            .insert(ident.to_string(), string_literal.value());
                                    }
                                    _ => panic!("Only string value is supported"),
                                },
                                _ => panic!("Path and List is not applicable"),
                            }
                        }
                    } else {
                        typemap.map.insert(ident.to_string(), ident.to_string());
                    }
                }
            }
            serde_json::to_string(&typemap.map).unwrap()
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
