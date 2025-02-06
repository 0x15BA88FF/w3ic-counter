use ic_cdk::storage;
use ic_cdk_macros::{query, update};

#[update]
fn increment() {
    let counter = storage::get_mut::<i32>().unwrap_or(0);
    storage::set(counter + 1);
}

#[update]
fn reset() {
    storage::set(0);
}

#[query]
fn get_counter() -> i32 {
    storage::get().unwrap_or(0)
}
