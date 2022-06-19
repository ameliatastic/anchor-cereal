use proc_macro::TokenStream;
use quote::quote;
use syn;

fn parse_array_ast(ast_data: syn::Data) -> (std::boxed::Box<syn::Type>, syn::Expr) {
  match ast_data {
    syn::Data::Struct(syn::DataStruct {
      fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
      ..
    }) => {
      if named.len() > 1 {
        todo!();
      }

      match named.first() {
        Some(syn::Field {
          ty: syn::Type::Array(syn::TypeArray { elem, len, .. }),
          ..
        }) => (elem.clone(), len.clone()),
        _ => panic!("Cannot derive: expected struct with a single field `value: [T; N]`"),
      }
    }
    _ => panic!("Cannot derive: expected struct with named fields"),
  }
}

#[proc_macro_derive(AnchorDefaultArray)]
pub fn anchor_default_array_derive(input: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(input).unwrap();
  let name = ast.ident;
  let (_, len) = parse_array_ast(ast.data);

  let gen = quote! {
      impl Default for #name {
          #[inline]
          fn default() -> Self {
            Self { value: [Default::default(); #len] }
          }
      }
  };

  gen.into()
}

#[proc_macro_derive(AnchorSerializeArray)]
pub fn anchor_serialize_array_derive(input: TokenStream) -> TokenStream {
  let ast: syn::DeriveInput = syn::parse(input).unwrap();
  let name = ast.ident;

  let gen = quote! {
      impl borsh::BorshSerialize for #name {
          #[inline]
          fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
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
  let (ty, len) = parse_array_ast(ast.data);

  let gen = quote! {
      impl borsh::BorshDeserialize for #name {
          #[inline]
          fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
            let items = Vec::with_capacity(#len);

            for i in 0..#len {
              items.push(#ty::deserialize(buf)?);
            }

            return Ok(Self { value: items.try_into().unwrap() });
          }
      }

      impl std::ops::Deref for #name {
          type Target = [#ty; #len];

          fn deref(&self) -> &Self::Target {
              &self.value
          }
      }

      impl std::ops::DerefMut for #name {
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
