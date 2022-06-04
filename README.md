# anchor-big-array

For some reason, Anchor doesn't let you create weird-sized arrays in #[account]-derived structs. This crate fixes that by providing a macro that generates a serializable struct with an array of any size.

## Usage

```rs
// Defines a struct that looks like:
// struct BigData { arr: [u8; 200] }
// and derives BorshSerialize/BorshDeserialize for it.
def_big_array!(BigData, u8, 200);

#[account]
pub struct  {
    data: BigData
}
```