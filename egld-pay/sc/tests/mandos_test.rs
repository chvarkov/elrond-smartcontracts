extern crate egldpay;
use egldpay::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

#[test]
fn test_mandos() {
	mandos_go("mandos/adder.scen.json");
}
