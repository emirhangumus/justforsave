mod types;

use ic_cdk_macros::export_candid;
use types::*;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use ic_cdk::export::Principal;

thread_local! {
    static SERVICE: RefCell<MainService> = RefCell::default();
}

#[derive(Default)]
pub struct MainService {
    pub users: HashSet<Person>,
    pub user_count: u32,
}


impl From<BasicServiceStorage> for MainService {
    fn from(stable: BasicServiceStorage) -> MainService {
        let accounts = stable.users.into_iter().map(|(_, v)| v).collect();

        MainService {
            users: accounts,
            user_count: stable.user_count,
        }
    }
}

//QUERIES
#[ic_cdk::query]
fn get_user_count() -> usize {
    SERVICE.with(|s| s.borrow().users.len())
}

#[ic_cdk::update]
fn add_new_user(name: String, age: u32) -> Result<(), String> {
    let user: Person = Person { name, age };
    SERVICE.with(|s| {
        let mut service = s.borrow_mut();
        service.users.insert(user);
    });
    Ok(())
}

#[ic_cdk::query]
fn get_list_users() -> Vec<Person> {

    SERVICE.with(|service: &RefCell<MainService>| {
        service.borrow().users.iter().map(|v: &Person| v.clone()).collect()
    })
}