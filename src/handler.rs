use gstd::{msg, ToString};
use io::{StoreAction, StoreError, StoreResponse};

use crate::{contract::StoreContract, get_store};

#[no_mangle]
extern "C" fn handle() {
    let action: StoreAction = msg::load().expect("unable to load message");
    let store: &mut StoreContract = get_store();

    if !store.store_admins.contains(&msg::source()) {
        let _ = msg::reply(Err::<StoreResponse, StoreError>(StoreError::NotAdmin), 0);
        return;
    }

    let result: Result<StoreResponse, StoreError> = match action {
        StoreAction::StoreTransaction { user, transtaction_type, amount } => {
            store.store_transaction(user, transtaction_type, amount)
        },
        StoreAction::StoreUnestake { user, amount, liberation_era, liberation_days } => {
            if let Err(err) = store.store_transaction(user, "unestake".to_string(), amount) {
                Err(err)
            } else {
                store.store_unestake(user, amount, liberation_era, liberation_days)
            }
        },
        StoreAction::FetchUnestake { user, unestake_id } => {
            store.fetch_unestake(user, unestake_id)
        },
        StoreAction::DeleteUnestake(unestake_id, user) => {
            store.delete_unestake(unestake_id, user)
        }
        StoreAction::AddAmin(admin) => {
            store.add_admin(admin)
        },
    };

    let _ = msg::reply(result, 0);
}