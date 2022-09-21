use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract_builder("file:output/empty.wasm", loan_data::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    elrond_wasm_debug::mandos_rs("mandos/empty.scen.json", world());
}