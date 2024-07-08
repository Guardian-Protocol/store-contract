#![no_std]
use gmeta::{In, InOut, Metadata};
use gstd::{ActorId, Decode, Encode, String, TypeInfo, Vec};
use state_io::{StoreQuery, StoreQueryResponse};

pub type TransactionId = u64;
pub type UnestakeId = u128;

pub type Gvara = u128;
pub type Vara = u128;

pub mod state_io;

#[derive(TypeInfo, Encode, Decode)] 
pub enum StoreAction {
    StoreTransaction {
        user: ActorId,
        transtaction_type: String,
        amount: Vara,
    },
    StoreUnestake {
        user: ActorId,
        amount: Vara,
        liberation_era: u64,
        liberation_days: u64,
    },
    DeleteUnestake(u128, ActorId),
    FetchUnestake {
        user: ActorId,
        unestake_id: UnestakeId,
    },
    AddAmin(ActorId),
}

#[derive(TypeInfo, Encode, Decode)]
pub enum StoreResponse {
    TransactionStored,
    UnestakeStored,
    Unestake {
        unestake: Unestake,
    },
    UnestakeDeleted,
    AdminAdded,
}

#[derive(TypeInfo, Encode, Decode)]
pub enum StoreError {
    UserNotFound,
    UnestakeNotFound,
    AdminAlreadyExists,
    NotAdmin,
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub struct Unestake {
    pub unestake_id: UnestakeId,
    pub amount: Gvara,
    pub liberation_era: u64,
    pub liberation_days: u64,
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub struct Transaction {
    pub transaction_id: TransactionId,
    pub transaction_type: String,
    pub amount: Vara,
}

#[derive(TypeInfo, Clone)]
pub struct UserInformation {
    pub user_total_vara_staked: u128,
    pub transaction_id_counter: u128,
    pub unestake_id_counter: u128,
    pub transactions: Vec<Transaction>,
    pub unestakes: Vec<Unestake>,
}

#[derive(TypeInfo, Encode, Decode)]
pub struct InitStore {
    pub admins: Vec<ActorId>,
}

pub struct StoreMetadata;

impl Metadata for StoreMetadata {
    type Init = In<InitStore>;
    type Handle = InOut<StoreAction, Result<StoreResponse, StoreError>>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = InOut<StoreQuery, Result<StoreQueryResponse, StoreError>>;
}