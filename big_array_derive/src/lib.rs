use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AnchorSerializeArray)]
pub fn anchor_serialize_array_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;

    let gen = quote! {
        impl BorshSerialize for #name {
            #[inline]
            fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
                for el in self.value.iter() {
                    el.serialize(writer)?;
                }
                Ok(())
            }
        }
    };

    gen.into()

}

#[proc_macro_derive(AnchorDeserializeArray)]
pub fn anchor_deserialize_array_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = ast.ident;
    let (ty, len) = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => {
            if named.len() > 1 { todo!(); }

            match named.first() {
                Some(syn::Field {
                    ty: syn::Type::Array(syn::TypeArray { elem, len, .. }),
                    ..
                }) => {
                    (elem.clone(), len.clone())
                }
                _ => panic!("Cannot derive: expected struct with a single field `value: [T; N]`")
            }
        },
        _ => panic!("Cannot derive: expected struct with named fields")
    };


    let gen = quote! {
        impl BorshDeserialize for #name {
            #[inline]
            fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
                let mut value = [#ty::default(); #len];
                if !<#ty>::copy_from_bytes(buf, &mut value)? {
                    for i in 0..#len {
                        value[i] = <#ty>::deserialize(buf)?;
                    }
                }
                Ok(#name { value })
            }
        }

        impl Deref for #name {
            type Target = [#ty; #len];

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.value
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(AnchorSerializeSkip)]
pub fn anchor_serialize_skip_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(AnchorDeserializeSkip)]
pub fn anchor_deserialize_skip_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}