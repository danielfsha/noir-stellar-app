#[oracle(fetchEthPrice)]
unconstrained fn get_price() -> Field { }

unconstrained fn eth_price() -> Field {
    get_price()
}

fn main() -> pub Field {
    let price = eth_price();
    assert(price > 0 && price < 1000000);  // Constrain oracle
    price
}

#[test]
fn test() { main(); }
