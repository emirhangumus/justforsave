mod types;

use ic_cdk_macros::export_candid;
use types::*;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::export::Principal;

thread_local! {
    static SERVICE: RefCell<MainService> = RefCell::default();
}

#[derive(Default)]
pub struct MainService {
    pub users: HashMap<Principal, Person>,
    pub user_count: u32,
    pub logs: Vec<String>,
}


impl From<BasicServiceStorage> for MainService {
    fn from(stable: BasicServiceStorage) -> MainService {
        let accounts = stable.users.into_iter().map(|(k, v)| (k, v)).collect();

        MainService {
            users: accounts,
            user_count: stable.user_count,
            logs: Vec::new(),
        }
    }
}

//QUERIES
#[ic_cdk::query]
fn get_user_count() -> u32 {
    SERVICE.with(|s| s.borrow().user_count)
}

#[ic_cdk::update]
fn add_new_user(name: String, age: u32) -> Result<(), String> {
    let user: Person = Person { name, age };
    SERVICE.with(|s| {
        let mut service = s.borrow_mut();
        let caller = ic_cdk::api::caller();
        service.users.insert(caller, user);
        service.user_count += 1;
    });
    Ok(())
}

#[ic_cdk::query]
fn get_list_users() -> Vec<Person> {
    SERVICE.with(|s| {
        let mut service = s.borrow_mut();
        let str2 = format!("get_list_users: {:?}", service.users);
        service.logs.push(str2);
        service.users.values().map(|v| v.clone()).collect()
    })
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn get_logs() -> Vec<String> {
    SERVICE.with(|s| {
        let mut service = s.borrow_mut();
        let str2 = format!("get_logs: {:?}", service.logs);
        service.logs.push(str2);
        service.logs.clone()
    })
}

export_candid!();