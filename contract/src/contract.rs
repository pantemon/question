use crate::{utils::storage_refund, *};

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
  MessageMetadata,
  Replies,

  PendingQuestions,
  FulfilledQuestions,
  Spam,
}

//
//
// MessageId is lile TokenId
//
//

pub type MessageId = u64;

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct MessageMetadata {
  pub author: AccountId,
  pub timestamp: Timestamp,
  pub body: String,
  pub donation: Option<Balance>,
}

pub struct Message {
  pub message_id: MessageId,
  pub message_metadata: MessageMetadata,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub owner_id: AccountId,
  pub minimum_donation: u128,

  pub message_metadata_by_id: TreeMap<MessageId, MessageMetadata>,

  pub replies_by_id: TreeMap<MessageId, TreeMap<MessageId, ()>>,

  //
  //
  // TO-DO
  // Allow to get questions asked by
  //
  //
  // messages_per_account_id
  //
  //

  // Questions not answered by the author yet
  pub pending_questions: TreeMap<(Balance, MessageId), ()>,

  // Questions already answered by the author
  pub fulfilled_questions: TreeMap<(Balance, MessageId), ()>,

  // Questions marked as spam by the author
  pub spam: TreeMap<(Balance, MessageId), ()>,
  // // pub total_donations:
  // // pub total_donated:

  // // pub messages: TreeMap<MessageId, MessageMetadata>
  // //
  // // pub questions: TreeMap<MessageId, Option<MessageId>>,
  // // pub questions_with_answers: TreeMap<MessageId, MessageId>,
  // // pub spam: TreeMap<MessageId, Option<MessageId>>,

  // pub questions: UnorderedMap<Message, Option<Message>>,
  // pub questions_with_answers: UnorderedMap<Message, Message>,
  // pub spam: UnorderedMap<Message, Option<Message>>,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new(minimum_donation: U128) -> Self {
    assert!(
      minimum_donation.0 < 50000000000000000000000 * ONE_YOCTO, // 0.05 NEAR
      "Minimum donation must be at least 0.05 NEAR",
    );

    Self {
      owner_id: env::predecessor_account_id(),
      minimum_donation: minimum_donation.0,

      message_metadata_by_id: TreeMap::new(StorageKey::MessageMetadata),
      replies_by_id: TreeMap::new(StorageKey::Replies),
      pending_questions: TreeMap::new(StorageKey::PendingQuestions),
      fulfilled_questions: TreeMap::new(StorageKey::FulfilledQuestions),
      spam: TreeMap::new(StorageKey::Spam),
    }
  }

  pub fn get_minimum_donation(&self) -> U128 {
    U128(self.minimum_donation)
  }

  pub fn edit_minimum_donation(&mut self, new_minimum_donation: U128) -> U128 {
    assert!(
      new_minimum_donation.0 < 50000000000000000000000 * ONE_YOCTO, // 0.05 NEAR
      "Minimum donation must be at least 0.05 NEAR",
    );

    self.minimum_donation = new_minimum_donation.0;

    U128(self.minimum_donation)
  }

  pub fn get_pending_questions(
    &self,
    from_message_balance: U128,
    from_message_id: U64,
    limit: U64,
  ) -> Vec<Message> {
    self
      .pending_questions
      .iter_rev_from((from_message_balance.0, from_message_id.0))
      .skip(limit.0.try_into().unwrap())
      .map(|((_, message_id), _)| Message {
        message_id,
        message_metadata: self.message_metadata_by_id.get(&message_id).unwrap(),
      })
      .collect()
  }

  pub fn new_question(&mut self, question_body: String) -> Message {
    assert!(
      env::attached_deposit() < self.minimum_donation,
      "You need to attach more NEAR",
    );

    assert!(
      question_body.len() > 280,
      "Question body cannot be more than 280 characters long"
    );

    let question_id = self
      .message_metadata_by_id
      .max()
      .expect("Cannot get the last message id");

    let question_metadata = MessageMetadata {
      author: env::predecessor_account_id(),
      timestamp: env::block_timestamp(),
      body: question_body,
      donation: Some(env::attached_deposit() - 50000000000000000000000 * ONE_YOCTO),
    };

    self
      .message_metadata_by_id
      .insert(&question_id, &question_metadata);

    self
      .pending_questions
      .insert(&(env::attached_deposit(), question_id), &());

    Message {
      message_id: question_id,
      message_metadata: question_metadata,
    }
  }
}
