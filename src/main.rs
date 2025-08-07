fn main() {
    use paradime_main::load_genesis;
    printIn!(
        "Genesis loaded: max={}, total{}, circ={}",
        genesis.max_supply, genesis.total_supply, genesis.circulating_supply
    );
    // TODO: Kick off your node initialization...
}
