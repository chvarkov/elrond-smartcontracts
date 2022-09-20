#![no_std]
// #![feature(array_methods)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod merchant_mod;
mod payment_mod;
mod storage_mod;

#[elrond_wasm::contract]
pub trait EgldPay:
	storage_mod::StorageModule +
	merchant_mod::MerchantModule +
	payment_mod::PaymentModule {

	#[view(getServiceFee)]
	fn get_service_fee(&self) -> u16 {
		self.service_fee().get()
	}

	#[init]
	fn init(&self) {
		self.init_merchant_module();
		self.init_payment_module(&150u16);

		// for debug
		// self.register_merchant();
	}

	#[endpoint(registerMerchant)]
	fn register_merchant(&self) -> SCResult<()> {
		let caller = &self.blockchain().get_caller();

		self.create_merchant(&caller);
		
		Ok(())
	}

	#[only_owner]
	#[endpoint(claimServiceReward)]
	fn claim_service_reward(&self) -> SCResult<()> {
		let available_amount = self
			.blockchain()
			.get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0);

		self.send()
			.direct_egld(&self.blockchain().get_owner_address(), &available_amount);

		Ok(())
	}
}
