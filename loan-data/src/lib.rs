#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use elrond_wasm::types::heap::BoxedBytes;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Eq, Clone, Debug)]
pub enum Status {
    Redeemed,
    NotRedeemed,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct LoanInfo<M: ManagedTypeApi> {
    pub id: BigUint<M>,
    pub amount: BigUint<M>,
    pub loaner_address: ManagedAddress<M>,
    pub token: EgldOrEsdtTokenIdentifier<M>,
    pub timestamp: u64,
    pub status: Status,
    pub note: BoxedBytes,
}

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct LoanShortInfo<M: ManagedTypeApi> {
    pub id: BigUint<M>,
    pub amount: BigUint<M>,
    pub loaner_address: ManagedAddress<M>,
    pub token: EgldOrEsdtTokenIdentifier<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct LoanCreateOptions<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub loaner_address: ManagedAddress<M>,
    pub token: EgldOrEsdtTokenIdentifier<M>,
    pub status: Status,
    pub note: BoxedBytes,
}

#[elrond_wasm::contract]
pub trait LoanDataContract {
    #[storage_mapper("index")]
    fn index(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("loans")]
    fn loans(&self, id: &BigUint) -> SingleValueMapper<LoanInfo<Self::Api>>;

    #[init]
    fn init(&self) {
        self.index().set(&BigUint::zero());
    }

    #[view(getById)]
    fn get_by_id(&self,
                 id: BigUint) -> SCResult<LoanInfo<Self::Api>> {
        let loan_mapper = self.loans(&id);

        require!(!loan_mapper.is_empty(), "Load record not found");

        Ok(loan_mapper.get())
    }

    #[view(getList)]
    fn get_list(&self,
                page: u64,
                size: u16) -> SCResult<MultiValueManagedVec<Self::Api, LoanShortInfo<Self::Api>>> {
        require!(page > 0, "Page must be greater than 0");
        require!(size >= 10 && size <= 50, "Size must be greater than 10 and less than 50");

        let offset: u64 = page - 1u64;

        let mut list: MultiValueManagedVec<Self::Api, LoanShortInfo<Self::Api>> = MultiValueManagedVec::new();

        for position in 1..size {
            let id: BigUint = BigUint::from(offset.clone()) + BigUint::from(position as u32);

            let loan_mapper = self.loans(&id);

            if loan_mapper.is_empty() {
                break;
            }

            let unwrapped = loan_mapper.get();

            let short: LoanShortInfo<Self::Api> = LoanShortInfo {
                id: unwrapped.id,
                token: unwrapped.token,
                amount: unwrapped.amount,
                loaner_address: unwrapped.loaner_address,
            };

            list.push(short);
        }

        Ok(list)
    }

    #[endpoint(create)]
    fn create(&self, data: LoanCreateOptions<Self::Api>) -> SCResult<LoanInfo<Self::Api>> {
        let id = self.increment_index();

        let loan: LoanInfo<Self::Api> = LoanInfo {
            id: id.clone(),
            amount: data.amount,
            loaner_address: data.loaner_address,
            token: data.token,
            status: data.status,
            note: data.note,
            timestamp: self.blockchain().get_block_timestamp(),
        };

        self.loans(&id).set(&loan);

        Ok(loan)
    }

    #[endpoint(createByArgs)]
    fn create_by_args(&self,
                      amount: BigUint,
                      loaner_address: ManagedAddress,
                      token: EgldOrEsdtTokenIdentifier,
                      status: Status,
                      note: BoxedBytes) -> SCResult<LoanInfo<Self::Api>> {
        let id = self.increment_index();

        let loan: LoanInfo<Self::Api> = LoanInfo {
            id: id.clone(),
            amount,
            loaner_address,
            token,
            status,
            note,
            timestamp: self.blockchain().get_block_timestamp(),
        };

        self.loans(&loan.id).set(&loan);

        Ok(loan)
    }

    #[endpoint(updateStatus)]
    fn update_status(&self, id: BigUint, status: Status) -> SCResult<Status> {
        let mut loan = self.get_by_id(id).unwrap();

        loan.status = status;

        self.loans(&loan.id).set(&loan);

        Ok(loan.status)
    }

    fn increment_index(&self) -> BigUint {
        let count_mapper = self.index();

        let id: BigUint = count_mapper.get();

        let next_id = id + BigUint::from(1u32);

        count_mapper.set(&next_id);

        next_id
    }
}
