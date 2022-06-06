pub use std::{
   io::{self, Write},
   ops::{Deref, DerefMut}
};
pub use borsh::{BorshDeserialize, BorshSerialize};

pub use big_array_derive::*;

pub trait AnchorSerializeArray: BorshSerialize {}
pub trait AnchorDeserializeArray: BorshDeserialize + Deref + DerefMut {}

/*
pub trait AbaAnchorDeserializeAnchorSerialize:
   BorshDeserialize +
   BorshSerialize +
   Deref +
   DerefMut {}
*/