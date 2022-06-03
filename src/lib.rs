pub use paste::paste;
pub use std::io::{self, Write};
pub use borsh::{BorshDeserialize, BorshSerialize};

#[macro_export]
macro_rules! big_array {
    [$T:ty ; $n:literal] => {
        paste! {
            [< __BigArray $T $n >]
        }
    }
}

#[macro_export]
macro_rules! def_big_array {
    [$T:ty ; $n:literal] => {
        paste! {
            #[derive(Clone)]
            pub struct [< __BigArray $T $n >] ([$T; $n]);

            impl BorshDeserialize for [< __BigArray $T $n >] {
                #[inline]
                fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
                    let mut result = [$T::default(); $n];
                    if !$T::copy_from_bytes(buf, &mut result)? {
                        for i in 0..$n {
                            result[i] = $T::deserialize(buf)?;
                        }
                    }
                    Ok([< __BigArray $T $n >](result))
                }
            }

            impl BorshSerialize for [< __BigArray $T $n >] {
                #[inline]
                fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
                    for el in self.0.iter() {
                        el.serialize(writer)?;
                    }
                    Ok(())
                }
            }
        }
    }
}