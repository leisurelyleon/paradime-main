use paradime_main::load_genesis;

fn main() {
    // Load the genesis configuration
    let genesis = load_genesis("genesis.json");

    // Print out the loaded supply parameters
    println!(
        "Genesis loaded: max={}, total={}, circ={}",
        genesis.max_supply,
        genesis.total_supply,
        genesis.circulating_supply
    );

    // TODO: Initialize node, start consensus engine, spin up RPC server…
}
