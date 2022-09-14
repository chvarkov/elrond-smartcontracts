elrond_wasm::imports!();

use crate::storage_mod;

#[elrond_wasm_derive::module]
pub trait MerchantModule: storage_mod::StorageModule {
    #[view(getMerchantAddress)]
    fn get_merchant_address(&self, merchant_id: &BigUint) -> SCResult<Address> {
        let merchant_mapper = self.merchants(merchant_id);

        require!(!merchant_mapper.is_empty(), "Merchant not found");

        Ok(merchant_mapper.get())
    }

    fn init_merchant_module(self) {
        self.merchant_count().set(&BigUint::zero());
    }

    fn create_merchant(self, address: &Address) {
        let merchant_id = self.reserve_merchant_id();

        self.merchants(&merchant_id).set(address);
    }

    fn reserve_merchant_id(&self) -> BigUint {
        let count_mapper = self.merchant_count();

        let id: BigUint = count_mapper.get();

        let next_id = id + BigUint::from(1u32);

        count_mapper.set(&next_id);

        next_id
    }
}
