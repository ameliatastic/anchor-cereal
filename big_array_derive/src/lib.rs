use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AbaAnchorDeserializeAnchorSerialize)]
pub fn abaadas_derive(input: TokenStream) -> TokenStream {
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
                _ => panic!("Cannot derive")
            }
        },
        _ => panic!("Cannot derive")
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

        impl BorshSerialize for #name {
            #[inline]
            fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
                for el in self.value.iter() {
                    el.serialize(writer)?;
                }
                Ok(())
            }
        }

        impl Index<usize> for #name {
            type Output = #ty;

            fn index(&self, idx: usize) -> &Self::Output {
                &self.value[idx]
            }
        }

        impl IndexMut<usize> for #name {
            fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                &mut self.value[idx]
            }
        }
    };

    gen.into()
}