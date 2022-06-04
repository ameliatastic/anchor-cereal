use proc_macro::TokenStream;

#[proc_macro_derive(NotAnchorSerialize)]
pub fn nas_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(NotAnchorDeserialize)]
pub fn nad_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
