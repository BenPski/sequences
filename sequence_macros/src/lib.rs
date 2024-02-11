//! Macros used for defining the more mechanical sequences
//! Expected that the macro generates the struct, Default trait, and the Iterator trait
//!

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{self, Ident, LitInt, parse_macro_input, Token, bracketed};
use syn::parse::{Parse, Error};

struct Additive {
    name: Ident,
    start: LitInt,
    step: LitInt,
}

impl Parse for Additive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let start: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let step: LitInt = input.parse()?;
        Ok(Additive { name , start , step })
    }
}

/// Create an additive sequence, expected usage is to provide the sequence
/// id the starting value and the step size
///
/// create_additive!(A001477, 0, 1);
/// The non-negative integers start at 0 and increase by 1 at every step
///
#[proc_macro]
pub fn create_additive(input: TokenStream) -> TokenStream {
    let Additive { name, start, step } = parse_macro_input!(input as Additive);

    let expanded = quote! {
        pub struct #name {
            value: num::BigInt,
        }

        impl Default for #name {
            fn default() -> Self {
                Self { value: (#start).into() }
            }
        }

        impl Iterator for #name {
            type Item = num::BigInt;

            fn next(&mut self) -> Option<Self::Item> {
                let orig = self.value.clone();
                self.value += #step;
                Some(orig)
            }
        }

    };

    TokenStream::from(expanded)
}

struct Recurrent {
    name: Ident,
    coeffs: Punctuated<LitInt, Token![,]>,
    start: Punctuated<LitInt, Token![,]>,
}

impl Parse for Recurrent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let _ = bracketed!(content in input);
        let coeffs = Punctuated::<LitInt, Token![,]>::parse_terminated(&content)?;
        input.parse::<Token![,]>()?;
        let content;
        let _ = bracketed!(content in input); 
        let start = Punctuated::<LitInt, Token![,]>::parse_terminated(&content)?;
        if coeffs.len() != start.len() {
            Err(Error::new(name.span(), "Expected coefficients array and value array to have the same length."))
        } else {
            Ok(Recurrent { name , coeffs, start })
        }
    }
}

/// Create a recurrent sequence (a sequence that depends on the previous values
/// of the sequence). Expected to provide the id of the sequence, the coefficients
/// of the recurrence relation, and the starting values.
///
/// create_recurrent!(A000045, [1, 1], [0, 1]);
/// The fibonacci sequence has the recurrence F_{n} = 1*F_{n-1} + 1*F_{n-2}
/// and starts with F_0 = 0 and F_1 = 1
///
#[proc_macro]
pub fn create_recurrent(input: TokenStream) -> TokenStream {
    let Recurrent { name, coeffs, start } = parse_macro_input!(input as Recurrent);
    

    let expanded = quote! {
        pub struct #name {
            coeffs: Vec<num::BigInt>,
            values: std::collections::VecDeque<num::BigInt>,
            index: usize,
        }

        impl Default for #name {
            fn default() -> Self {
                Self { 
                    coeffs: vec![#coeffs].into_iter().map(|x| x.into()).collect::<Vec<_>>(),
                    values: std::collections::VecDeque::from_iter(vec![#start].into_iter().map(|x| x.into())),
                    index: 0,
                }
            }
        }

        impl Iterator for #name {
            type Item = num::BigInt;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index < self.values.len()-1 {
                    let index = self.index;
                    self.index += 1;
                    Some(self.values[index].clone())
                } else {
                    let orig = self.values[self.values.len()-1].clone();
                    let mut val = 0.into();
                    for i in 0..self.coeffs.len() {
                        val += &self.coeffs[i]*&self.values[i];
                    }
                    self.values.pop_front();
                    self.values.push_back(val);
                    Some(orig)
                }
            }
        }

    };

    TokenStream::from(expanded)
}

