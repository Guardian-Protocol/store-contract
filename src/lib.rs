#![no_std]

use contract::StoreContract;
use gstd::{msg, ToString};
use io::InitStore;

mod contract;
mod handler;
mod state;

static mut STORE: Option<StoreContract> = None;

pub fn get_store() -> &'static mut StoreContract {
    unsafe {
        STORE.as_mut().unwrap()
    }
}

#[no_mangle]
extern "C" fn init() {
    let init_config: InitStore = msg::load().expect("unable to load message");

    unsafe {
        STORE = Some(StoreContract {
            store_admins: init_config.admins,
            ..Default::default()
        })
    }

    let _ = msg::reply("Store contract initialized".to_string(), 0);
}