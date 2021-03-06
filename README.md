# anchor-cereal

A quality of life package for Anchor made to support the Solana ecosystem. Currently, it allows you to serialize a struct containing an array of any size, while still showing up in your IDL. More features coming soon!! :)

## Usage

```rs
use anchor_cereal::array::*;

...

// Define a struct to wrap your array - it must:
//   - derive Clone, AnchorSerializeArray, and AnchorDeserializeArray
//   - contain a single field named `value`, which is an array.
#[derive(Clone, AnchorSerializeArray, AnchorDeserializeArray)]
pub struct BigData {
    value: [u8; 50]
}

// You can then use your struct in accounts...
#[account]
pub struct MyAccount {
    data: BigData
}

...

// ...and instructions.
pub fn initialize(
   ctx: Context<Initialize>,
   data: BigData
) -> Result<()> {
   let my_account = &mut ctx.accounts.my_account;

   my_account.data = data;

   // BigData gets implementations for Deref and DerefMut via the deserialize
   // trait, so you don't have to reference the inner `value` in order to use
   // the array.
   let first = my_account.data[0];
   my_account.data[1] = 2;
   // Same as:
   //   let first = my_account.data.value[0];
   //   my_account.data.value[1] = 2;

   ...

   Ok(())
}
```
