use crate::*;

#[near_bindgen]
impl Contract {
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

    pub fn get_all_cashout_requests(&self) -> Vec<AccountId> {
        return vector_to_vec(&self.cashout_requests.keys_as_vector());
    }

    //

    pub fn approve_cashout_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);

        let amount = self.cashout_requests.get(&account).unwrap();
        self.cashout_requests.remove(&account);
        self.token.internal_withdraw(account, amount);
    }

    pub fn decline_cashout_request(&mut self, account: &AccountId) {
        self.only_role(ROLES::Manager as u8);

        self.cashout_requests.remove(&account);
    }
}
