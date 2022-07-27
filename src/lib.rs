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

    //

    pub fn claim(&mut self, amount: Balance) {
        if amount < 0 as Balance {
            env::panic_str("The amount should be a positive number");
        }

        if let Some(_i) = self.claim_requests.get(&env::predecessor_account_id()) {
            env::panic_str("Already have request for claim");
        } else {
            self.claim_requests.insert(&env::predecessor_account_id(), &amount);    
        }
    }

    pub fn cancel_claim(&mut self) {
        if let Some(_i) = self.claim_requests.get(&env::predecessor_account_id()) {
            self.claim_requests.remove(&env::predecessor_account_id());    
        } else {
            env::panic_str("Have no requests rn");
        }
    }

    pub fn cashout(&mut self, amount: Balance) {
        if let Some(_i) = self.cashout_requests.get(&env::predecessor_account_id()) {
            env::panic_str("Already have request for claim");
        }

        if self.token.internal_unwrap_balance_of(&env::predecessor_account_id()) < amount {
            env::panic_str("Not enough funds");
        }

        self.cashout_requests.insert(&env::predecessor_account_id(), &amount);
    }

    pub fn cancel_cashout(&mut self) {
        if let Some(_i) = self.cashout_requests.get(&env::predecessor_account_id()) {
            self.cashout_requests.remove(&env::predecessor_account_id());    
        } else {
            env::panic_str("Have no requests rn");
        }
    }

    //

    pub fn get_all_claim_requests(&self) -> Vec<AccountId> {
        return vector_to_vec(&self.claim_requests.keys_as_vector());
    }

    pub fn get_all_cashout_requests(&self) -> Vec<AccountId> {
        return vector_to_vec(&self.cashout_requests.keys_as_vector());
    }

    //

    pub fn approve_claim_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);

        let amount = self.claim_requests.get(&account).unwrap();
        self.claim_requests.remove(&account);
        self.token.internal_deposit(account, amount);
    }

    pub fn approve_cashout_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);

        let amount = self.cashout_requests.get(&account).unwrap();
        self.cashout_requests.remove(&account);
        self.token.internal_withdraw(account, amount);
    }

    pub fn decline_claim_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);

        self.claim_requests.remove(&account);
    }

    pub fn decline_cashout_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);

        self.cashout_requests.remove(&account);
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
