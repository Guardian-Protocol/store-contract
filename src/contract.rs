use gstd::{
    collections::HashMap, vec, ActorId, String, Vec
};
use io::{
    StoreError, StoreResponse, Transaction, Unestake, UnestakeId, UserInformation, Vara
};

#[derive(Default)]
pub struct StoreContract {
    pub users: HashMap<ActorId, UserInformation>,
    pub store_admins: Vec<ActorId>,
    pub protocol_total_vara_staked: Vara,
}

impl StoreContract {

    pub fn store_transaction(
        &mut self, 
        user: ActorId, 
        t_type: String, 
        amount: Vara
    ) -> Result<StoreResponse, StoreError> {
        self.users.entry(user).and_modify(|u| {
            if t_type == "stake" {
                u.user_total_vara_staked += amount;
                self.protocol_total_vara_staked += amount;
            }

            u.transactions.push(Transaction {
                transaction_id: 0,
                transaction_type: t_type.clone(),
                amount,
            });

            u.transaction_id_counter += 1;
        }).or_insert(UserInformation {
            transaction_id_counter: 1,
            unestake_id_counter: 0,
            user_total_vara_staked: 0,
            transactions: vec![
                Transaction {
                    transaction_id: 0,
                    transaction_type: t_type.clone(),
                    amount,
                }
            ],
            unestakes: Vec::new(),
        });

        Ok(StoreResponse::TransactionStored)
    }

    pub fn store_unestake(
        &mut self, 
        user: ActorId, 
        amount: Vara, 
        liberation_era: u64, 
        liberation_days: u64
    ) -> Result<StoreResponse, StoreError>  {
        if let Some(user) = self.users.get(&user) {
            if user.user_total_vara_staked < amount {
                return Err(StoreError::InssuficientBalance);
            }
        } else {
            return Err(StoreError::UserNotFound);
        }

        let mut id = 0;
        self.users.entry(user).and_modify(|u| {
            u.unestakes.push(Unestake {
                unestake_id: u.unestake_id_counter,
                amount,
                liberation_era,
                liberation_days,
                interest_percent: amount / self.protocol_total_vara_staked * 100,
            });

            id = u.unestake_id_counter;
            u.unestake_id_counter += 1;
        });

        self.protocol_total_vara_staked -= amount;
        Ok(StoreResponse::UnestakeStored(id))
    }

    pub fn delete_unestake(
        &mut self, 
        user: ActorId, 
        unestake_id: UnestakeId
    ) -> Result<StoreResponse, StoreError>  {
        if let Some(user) = self.users.get_mut(&user) {
            if let Some(index) = user.unestakes.clone().into_iter().position(|u| u.unestake_id == unestake_id) {
                user.unestakes.remove(index);
                Ok(StoreResponse::UnestakeDeleted)
            } else {
                Err(StoreError::UnestakeNotFound)
            }
        } else {
            Err(StoreError::UserNotFound)
        }
    }

    pub fn fetch_unestake(
        &mut self, 
        user: ActorId, 
        unestale_id: UnestakeId
    ) -> Result<StoreResponse, StoreError>  {
        if let Some(user) = self.users.get(&user) {
            if let Some(unestake) = user.unestakes.clone().into_iter().find(|u| u.unestake_id == unestale_id) {
                Ok(StoreResponse::Unestake { unestake: unestake.clone() })
            } else {
                Err(StoreError::UnestakeNotFound)
            }
        } else {
            Err(StoreError::UserNotFound)
        }
    }

    pub fn add_admin(
        &mut self, 
        actor_id: ActorId
    ) -> Result<StoreResponse, StoreError> {
        if self.store_admins.contains(&actor_id) {
            Err(StoreError::AdminAlreadyExists)
        } else {
            self.store_admins.push(actor_id);
            Ok(StoreResponse::AdminAdded)
        }
    }

}