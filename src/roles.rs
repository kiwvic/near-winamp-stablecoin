use crate::*;

pub enum ROLES {
    Manager = 1,
}

#[near_bindgen]
impl Contract {
    pub fn only_role(&self, role: u8) {
        if self.roles.get(&env::predecessor_account_id()).unwrap() != role {
            env::panic_str("Wrong role");
        }
    }

    #[private]
    pub fn add_role(&mut self, account: &AccountId, role: u8) {
        if !self.roles.contains_key(&account) {
            env::panic_str("Account already has this role");
        }

        self.roles.insert(&account, &role);
    }
}
