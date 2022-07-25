use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, Balance, AccountId, PanicOnDefault, PromiseOrValue};
use near_sdk::serde::{Serialize, Deserialize};

mod roles;

use crate::roles::*;

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct CashoutOrClaimEvent {
//     pub account: AccountId,
//     pub amount: Balance,
// }

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

        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
            roles: LookupMap::new(b"r".to_vec()),
            claim_requests: UnorderedMap::new(b"cl".to_vec()),
            cashout_requests: UnorderedMap::new(b"ca".to_vec()),
        };

        this
    }


    pub fn mint(&mut self, account: &AccountId, amount: Balance) {
        self.only_role(ROLES::Manager as u8);

        self.token.internal_deposit(account, amount);
    }

    pub fn burn(&mut self, account: &AccountId, amount: Balance) {
        self.only_role(ROLES::Manager as u8);

        self.token.internal_withdraw(account, amount);
    }

    //

    pub fn claim(&mut self, amount: Balance) {
        assert!(amount > 0, "The amount should be a positive number");
        assert!(self.claim_requests.get(&env::predecessor_account_id()) != None, "Already have request");

        self.claim_requests.insert(&env::predecessor_account_id(), &amount);
    }

    pub fn cancel_claim(&mut self) {
        assert!(self.claim_requests.get(&env::predecessor_account_id()) == None, "Have no requests rn");

        self.claim_requests.remove(&env::predecessor_account_id());
    }

    pub fn cashout(&mut self, amount: Balance) {
        assert!(self.cashout_requests.get(&env::predecessor_account_id()) != None, "Already have request for cashout");
        assert!(self.token.internal_unwrap_balance_of(&env::predecessor_account_id()) < amount, "Not enough funds");

        self.cashout_requests.insert(&env::predecessor_account_id(), &amount);
    }

    pub fn cancel_cashout(&mut self) {
        assert!(self.cashout_requests.get(&env::predecessor_account_id()) == None, "Have no requests rn");

        self.cashout_requests.remove(&env::predecessor_account_id());
    }

    //

    pub fn get_all_claim_requests(&self) -> &Vector<AccountId> {
        return self.claim_requests.keys_as_vector();
    }

    pub fn get_all_cashout_requests(&self) -> &Vector<AccountId> {
        return self.claim_requests.keys_as_vector();
    }

    //

    pub fn approve_claim_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);
    }

    pub fn decline_claim_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);
    }

    pub fn approve_cashout_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);
    }

    pub fn decline_cashout_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);
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
