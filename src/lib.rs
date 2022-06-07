pub mod array {
   pub use anchor_cereal_derive::*;
   pub use std::{
      io::{self, Write},
      ops::{Deref, DerefMut}
   };
   pub use borsh::{BorshDeserialize, BorshSerialize};

   pub trait AnchorSerializeArray: BorshSerialize {}
   pub trait AnchorDeserializeArray: BorshDeserialize + Deref + DerefMut {}
}

pub mod skip {
   pub use anchor_cereal_derive::*;
   pub use std::{
      io::{self, Write},
      ops::{Deref, DerefMut}
   };
   pub use borsh::{BorshDeserialize, BorshSerialize};

   pub trait AnchorSerializeSkip {}
   pub trait AnchorDeserializeSkip {}
}
