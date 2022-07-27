use crate::*;

#[near_bindgen]
impl Contract {
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

    //

    pub fn get_all_claim_requests(&self) -> Vec<AccountId> {
        return vector_to_vec(&self.claim_requests.keys_as_vector());
    }

    //

    pub fn approve_claim_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);

        let amount = self.claim_requests.get(&account).unwrap();
        self.claim_requests.remove(&account);
        self.token.internal_deposit(account, amount);
    }

    pub fn decline_claim_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);

        self.claim_requests.remove(&account);
    }
}
