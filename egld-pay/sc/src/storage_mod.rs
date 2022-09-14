elrond_wasm::imports!();

#[elrond_wasm_derive::module]
pub trait StorageModule {
    #[storage_mapper("merchant_count")]
    fn merchant_count(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("merchants")]
    fn merchants(&self, merchant_id: &BigUint) -> SingleValueMapper<Address>;

    #[storage_mapper("service_fee")]
    fn service_fee(&self) -> SingleValueMapper<u16>;
}
