use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};


#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct BasicServiceStorage {
    pub users: HashMap<Principal, Person>,
    pub user_count: u32,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
}
