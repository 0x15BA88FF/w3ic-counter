use candid::types::number::Nat;
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0_u32));
}

#[ic_cdk_macros::query]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

#[ic_cdk_macros::update]
fn set(n: Nat) {
    COUNTER.with(|count| *count.borrow_mut() = n);
}

#[ic_cdk_macros::update]
fn reset() {
    COUNTER.with(|count| *count.borrow_mut() = 0);
}

#[ic_cdk_macros::update]
fn increase() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1_u32);
}
