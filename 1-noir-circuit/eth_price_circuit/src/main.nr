#[oracle(fetchEthPrice)]
unconstrained fn get_price() -> Field { }

unconstrained fn eth_price() -> Field {
    get_price()
}

fn main() -> pub Field {
    // Safety: Oracle calls are external and unconstrained
    let price = unsafe { eth_price() };
    let price_u64 = price as u64;
    assert((price_u64 > 0) & (price_u64 < 1000000));  // Constrain oracle
    price
}

#[test]
fn test() {
    let _ = main();
}
