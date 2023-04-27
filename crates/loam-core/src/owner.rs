use loam_sdk::{
    loam,
    soroban_sdk::{self, contracttype, Address, IntoKey, Lazy},
};

#[contracttype]
#[derive(IntoKey, Default)]
pub struct Owner(Kind);

/// Work around not having `Option` in `contracttype`
#[contracttype]
#[derive(Default)]
pub enum Kind {
    Address(Address),
    #[default]
    None,
}

impl IsOwnable for Owner {
    fn owner_get(&self) -> Option<Address> {
        match &self.0 {
            Kind::Address(address) => Some(address.clone()),
            Kind::None => None,
        }
    }

    fn owner_set(&mut self, new_owner: Address) {
        self.0 = Kind::Address(new_owner);
    }
}

#[loam]
pub trait IsOwnable {
    /// Get current owner
    fn owner_get(&self) -> Option<Address>;
    /// Transfer ownership if already set.
    /// Should be called in the same transaction as deploying the contract to ensure that
    /// a different account doesn't claim ownership
    fn owner_set(&mut self, new_owner: Address);
}
