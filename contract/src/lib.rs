use near_sdk::{
  borsh::{self, *},
  collections::*,
  json_types::*,
  serde::{self, *},
  *,
};

use std::ops::Bound;

mod utils;

mod contract;
pub use contract::*;

#[cfg(test)]
mod tests {
  use std::vec;

  use crate::*;
  use near_sdk::{test_utils::*, testing_env};

  const ONE_NEAR: u128 = u128::pow(10, 24);

  fn contract_account() -> AccountId {
    "contract".parse::<AccountId>().unwrap()
  }

  fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
      .current_account_id(contract_account())
      .account_balance(15 * ONE_NEAR)
      .signer_account_id(predecessor_account_id.clone())
      .predecessor_account_id(predecessor_account_id);
    builder
  }

  #[test]
  fn test() {
    let mut questions = TreeMap::<(Balance, MessageId), String>::new(b"q");

    questions.insert(&(1, 0), &"Do you like pizza?".into());
    questions.insert(&(1, 5), &"Do you like pizza?".into());
    questions.insert(&(2, 3), &"Do you like pizza?".into());
    questions.insert(&(3, 4), &"Do you like pizza?".into());
    questions.insert(&(4, 2), &"Do you like apples?".into());
    questions.insert(&(4, 1), &"Do you like pizza?".into());

    questions.iter_rev().for_each(|x| println!("{:?}", x))
  }
}
