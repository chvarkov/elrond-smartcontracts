elrond_wasm::imports!();

use crate::storage_mod;

#[elrond_wasm_derive::module]
pub trait PaymentModule: storage_mod::StorageModule {
    fn init_payment_module(self, default_fee: &u16) {
        self.service_fee().set(default_fee);
    }

    #[endpoint(payment)]
    #[payable("EGLD")]
    fn payment(&self,
               merchant_id: &BigUint,
               order_id: &BigUint,
               #[payment] amount: BigUint) -> SCResult<()> {
        let merchant_mapper = self.merchants(merchant_id);

        require!(!merchant_mapper.is_empty(), "Merchant not found");

        let merchant_address = merchant_mapper.get().managed_into();

        let service_fee = self.service_fee().get();

        let payment_amount = amount.clone() - (amount.clone() / service_fee as u32);

        self.send()
            .direct_egld(&merchant_address, &payment_amount, &[]);

        self.payment_event(
            &self.blockchain().get_caller().to_address(),
            merchant_id,
            order_id
        );

        Ok(())
    }

    #[event("payment")]
    fn payment_event(&self,
                     #[indexed] buyer: &Address,
                     #[indexed] merchant_id: &BigUint,
                     #[indexed] order_id: &BigUint,
    );
}
