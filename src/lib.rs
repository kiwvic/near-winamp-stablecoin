use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, Balance, AccountId, PanicOnDefault, PromiseOrValue};

mod roles;
mod utils;
mod claim;
mod cashout;

use crate::utils::*;
use crate::roles::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    roles: LookupMap<AccountId, u8>,

    claim_requests: UnorderedMap<AccountId, Balance>,
    cashout_requests: UnorderedMap<AccountId, Balance>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        metadata.assert_valid();

        Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
            roles: LookupMap::new(b"r".to_vec()),
            claim_requests: UnorderedMap::new(b"cl".to_vec()),
            cashout_requests: UnorderedMap::new(b"ca".to_vec()),
        }
    }

    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        // TODO panic if not artist

        self.token.storage_deposit(account_id, registration_only)
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token);
near_contract_standards::impl_fungible_token_storage!(Contract, token);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}
