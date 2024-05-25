use gstd::{ActorId, Decode, Encode, TypeInfo, Vec};

use crate::{Transaction, Unestake, Vara};

#[derive(TypeInfo, Encode, Decode)]
pub enum StoreQuery {
    GetUserVaraLocked(ActorId),
    GetTransactionHistory(ActorId),
    GetUnestakeHistory(ActorId),
}

#[derive(TypeInfo, Encode, Decode)]
pub enum StoreQueryResponse {
    UserVaraLocked(Vara),
    TransactionHistory(Vec<Transaction>),
    UnestakeHistory(Vec<Unestake>),
}