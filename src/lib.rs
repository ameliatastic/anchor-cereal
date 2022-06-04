pub use std::io::{self, Write};
pub use borsh::{BorshDeserialize, BorshSerialize};

pub trait NotAnchorSerialize {}
pub trait NotAnchorDeserialize {}

#[macro_export]
macro_rules! def_big_array {
    ($name:ident, $T:ty, $n:literal) => {
        //#[derive(Clone, NotAnchorSerialize, NotAnchorDeserialize)]
        //pub struct $name {
        //    pub arr: [$T; $n]
        //}
        #[derive(Clone)]
        pub enum $name {
            Arr([$T; $n])
        }

        impl BorshDeserialize for $name {
            #[inline]
            fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
                let mut arr = [<$T>::default(); $n];
                if !<$T>::copy_from_bytes(buf, &mut arr)? {
                    for i in 0..$n {
                        arr[i] = <$T>::deserialize(buf)?;
                    }
                }
                Ok($name::Arr(arr))
            }
        }

        impl BorshSerialize for $name {
            #[inline]
            fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
                if let $name::Arr(arr) = self {
                    for el in arr.iter() {
                        el.serialize(writer)?;
                    }
                }
                Ok(())
            }
        }
    }
}