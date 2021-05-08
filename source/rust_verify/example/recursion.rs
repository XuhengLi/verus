extern crate builtin;
use builtin::*;
mod pervasive;
use pervasive::*;

fn main() {}

#[spec]
fn arith_sum_int(i: int) -> int {
    decreases(i);

    if i <= 0 { 0 } else { i + arith_sum_int(i - 1) }
}

#[spec]
fn arith_sum_nat(i: nat) -> nat {
    decreases(i);

    if i == 0 { 0 } else { i + arith_sum_nat(i - 1) }
}

#[spec]
fn arith_sum_u64(i: u64) -> u64 {
    decreases(i);

    if i == 0 { 0 } else { i + arith_sum_u64(i - 1) }
}

#[proof]
fn arith_sum_int_nonneg(i: nat) {
    ensures(arith_sum_int(i) >= 0);
    decreases(i);

    if i > 0 {
        arith_sum_int_nonneg(i - 1);
    }
}

#[proof]
fn arith_sum_test1() {
    assert(arith_sum_int(0) == 0);
    assert(arith_sum_int(1) == 1);
    assert(arith_sum_int(2) == 3);
    assert(arith_sum_int(3) == 6);
}