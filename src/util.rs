use crate::*;

pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Required attached deposite at least 1 yoctorNEAR"
    );
}

pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Required attached deposit of exactly 1 Yocto NEAR"
    );
}

pub(crate) fn refund_deposit(storage_used: u64) {
    let require_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();

    assert!(
        attached_deposit >= require_cost,
        "Must attached {} yoctoNEAR to cover storage",
        require_cost
    );

    let refund = attached_deposit - require_cost;
    if refund > 0 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}
