//! Derives for the `tatk` crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// An internal value used to calculate additional details on composite indicators.
#[proc_macro_derive(InternalValue)]
pub fn internal_value_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the InternalValue trait.
    TokenStream::from(quote! {
        impl InternalValue for #struct_name {
            fn internal_value(&self) -> Num {
                self.value
            }
        }
    })
}

/// Enables the `period()` method. Period is the window of data to process.
#[proc_macro_derive(Period)]
pub fn period_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Period trait.
    TokenStream::from(quote! {
        impl Period for #struct_name {
            fn period(&self) -> usize {
                self.period
            }
        }
    })
}

/// Enables the `open()` method. Returns the opening value for the candle.
#[proc_macro_derive(Open)]
pub fn open_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Open trait.
    TokenStream::from(quote! {
        impl Open for #struct_name {
            fn open(&self) -> Num {
                self.open
            }
        }
    })
}

/// Enables the `close()` method. Returns the closing value for the candle.
#[proc_macro_derive(Close)]
pub fn close_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Close trait.
    TokenStream::from(quote! {
        impl Close for #struct_name {
            fn close(&self) -> Num {
                self.close
            }
        }
    })
}

/// Enables the `low()` method. Returns the lowest value for the candle.
#[proc_macro_derive(Low)]
pub fn low_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Low trait.
    TokenStream::from(quote! {
        impl Low for #struct_name {
            fn low(&self) -> Num {
                self.low
            }
        }
    })
}

/// Enables the `high()` method. Returns the highest value for the candle.
#[proc_macro_derive(High)]
pub fn high_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the High trait.
    TokenStream::from(quote! {
        impl High for #struct_name {
            fn high(&self) -> Num {
                self.high
            }
        }
    })
}

/// Enables the `volume()` method. Returns the volume value for the candle.
#[proc_macro_derive(Volume)]
pub fn volume_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Volume trait.
    TokenStream::from(quote! {
        impl Volume for #struct_name {
            fn volume(&self) -> Num {
                self.volume
            }
        }
    })
}
