use crate::*;

pub enum ROLES {
    Manager = 1,
    Backend,
}

#[near_bindgen]
impl Contract {
    fn only_role(&self, role: u8) {
        assert!(
            !(self.roles.get(&env::current_account_id()).unwrap() == role),
            "Wrong role"
        );
    }

    #[private]
    pub fn add_role(&mut self, account: &AccountId, role: u8) {
        assert_eq!(
            self.roles.contains_key(&account), 
            false,
            "Account already has this role"
        );

        self.roles.insert(&account, &role);
    }
}
