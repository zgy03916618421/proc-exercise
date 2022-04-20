//! learn from tyrchen
//! practice to write simple procedural macros,
//! just implement printing field of struct
//!
//! # Example
//! ```
//! [derive(MySerialize)]
//! struct User {
//!     name: String,
//!     age: u32,
//! }
//! fn main() {}
//! ```
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(MySerialize)]
pub fn derive_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    SerializeContext::from(input).render().into()
}

/// this struct mapping/extracting from deriveinput
/// so custom struct just contains data that we care,
/// and define our behaviour on itself. decouple data
/// from behaviour
///
/// # Errors
///
/// Panic an Error [`Unsupport data type`] if struct
/// don't contain any field
///
/// # Example
/// ```
///   let input = parse_macro_input!(input as DeriveInput);
///   let serialize_ctx = SerializeContext::from(input);
/// ```
///
///
struct SerializeContext {
    name: Ident,
    fields: Vec<Ident>,
}

impl From<DeriveInput> for SerializeContext {
    fn from(d: DeriveInput) -> Self {
        let name = d.ident;

        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = d.data
        {
            named
        } else {
            panic!("Unsupported data type");
        };

        let fds = fields
            .into_iter()
            .map(|f| f.ident.unwrap())
            .collect::<Vec<_>>();

        Self { name, fields: fds }
    }
}

impl SerializeContext {
    /// transform input to another stream that we need
    pub fn render(&self) -> TokenStream {
        let name = &self.name;
        let fields_print = self.gen_field_print();

        quote! {
            impl #name {
                fn serialize(&self) {
                    #(#fields_print)*
                }
            }
        }
    }

    /// for every field of struct, generate "println!" tokenstream
    fn gen_field_print(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|f| {
                let field_name = f.to_string();
                quote! {
                    println!("{}: {}", #field_name, self.#f);
                }
            })
            .collect()
    }
}
