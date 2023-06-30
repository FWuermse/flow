use core::panic;

use proc_macro::TokenStream;
use syn::{DeriveInput, Type};

fn impl_connectable_trait(ast: DeriveInput) -> TokenStream {
    // The struct name is required to know what Struct to impl the trait for.
    let struct_ident = ast.ident;
    let conn_field = match ast.data {
        syn::Data::Struct(s) => Some(s.fields.into_iter().filter(|f| {
            match &f.ty {
                Type::Path(p) => p
                    .path
                    .segments
                    .first()
                    .is_some_and(|f| f.ident.to_string() == "Connection"),
                _ => false,
            }
        })),
        _ => None,
    };
    // The nested connection field is required to know what field to delegate the impl to.
    match conn_field.unwrap().next().clone() {
        None => panic!("Your Struct requires a field of type Connection<I, O> in order to derive from Connectable.\n
        Example:\n
        pub struct MyNode<I, O> {{conn: Connection<I, O>, ...}}\n
        Alternatively you can implement the Connectable trait manually without using this macro."),
        Some(field) => {
            let field_ident = field.ident;
            quote::quote! {
                use crate::nodes::connection::ConnectError;
                use super::connection::Connection;
                use std::sync::mpsc::Sender;
                impl<I, O> Connectable<I, O> for #struct_ident<I, O> {
                    fn inputs(&self) -> &Vec<Sender<I>> {
                        &self.#field_ident.inputs()
                    }

                    fn output(&self) -> &Vec<Sender<O>> {
                        &self.#field_ident.output()
                    }

                    fn chain(&mut self, successors: Vec<std::sync::mpsc::Sender<O>>) -> &Self {
                        self.#field_ident.chain(successors);
                        self
                    }

                    fn send_at(&self, index: usize, value: I) -> Result<(), ConnectError<I>> {
                        self.#field_ident.send_at(index, value)
                    }

                    fn send(&self, value: I) -> Result<(), ConnectError<I>> {
                        self.#field_ident.send(value)
                    }

                    fn input_at(&self, index: usize) -> Result<Sender<I>, ConnectError<I>> {
                        self.#field_ident.input_at(index)
                    }

                    fn input(&self) -> Result<Sender<I>, ConnectError<I>> {
                        self.input_at(0)
                    }
                }
            }
            .into()
        }
    }
}

#[proc_macro_derive(Connectable)]
pub fn connectable_derive_macro(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();

    impl_connectable_trait(ast)
}
