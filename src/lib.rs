pub mod array {
  pub use anchor_cereal_derive::{
    AnchorDefaultArray, AnchorDeserializeArray, AnchorSerializeArray,
  };

  pub trait AnchorDefaultArray: Default {}
  pub trait AnchorSerializeArray: borsh::BorshSerialize {}
  pub trait AnchorDeserializeArray:
    borsh::BorshDeserialize + std::ops::Deref + std::ops::DerefMut
  {
  }
}

pub mod skip {
  pub use anchor_cereal_derive::{AnchorDeserializeSkip, AnchorSerializeSkip};

  pub trait AnchorSerializeSkip {}
  pub trait AnchorDeserializeSkip {}
}
