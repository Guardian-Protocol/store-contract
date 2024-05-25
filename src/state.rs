use gstd::msg;
use io::{state_io::{StoreQuery, StoreQueryResponse}, StoreError};

use crate::{
    contract::StoreContract, 
    get_store
};

#[no_mangle]
extern "C" fn state() {
    let query: StoreQuery = msg::load().expect("unable to load message");
    let store: &mut StoreContract = get_store();

    let result = match query {
        StoreQuery::GetUserVaraLocked(actor_id) => {
            if let Some(user) = store.users.get(&actor_id) {
                Ok(StoreQueryResponse::UserVaraLocked(user.user_total_vara_staked))
            } else {
                Err(StoreError::UserNotFound)
            }
        },
        StoreQuery::GetTransactionHistory(actor_id) => {
            if let Some(user) = store.users.get(&actor_id) {
                Ok(StoreQueryResponse::TransactionHistory(user.transactions.clone()))
            } else {
                Err(StoreError::UserNotFound)
            }
        },
        StoreQuery::GetUnestakeHistory(actor_id) => {
            if let Some(user) = store.users.get(&actor_id) {
                Ok(StoreQueryResponse::UnestakeHistory(user.unestakes.clone()))
            } else {
                Err(StoreError::UserNotFound)
            }
        },
    };

    let _ = msg::reply(result, 0);
}