#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, AccessControl, PSP22Mintable, PSP22Metadata)]
#[openbrush::contract]
pub mod dao_token {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DaoToken {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        metadata: metadata::Data,
    }
    const MINTER: RoleType = ink::selector_id!("MINTER");
    const MANAGER: RoleType = ink::selector_id!("MANAGER");
    #[default_impl(PSP20Mintable)]
    #[modifiers(only_role(MINTER))]
    fn mint() {}
    #[ink::trait_definition]
    pub trait TokenTrait {
        /// Returns the total token supply.
        #[ink(message)]
        fn total_supply(&self) -> Balance;

        /// Returns the account balance for the specified `owner`.
        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance;

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance);

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance);
        // mint
        #[ink(message)]
        fn mint(&mut self, to: AccountId, value: Balance);
        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance);
    }
    impl DaoToken {
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut _instance = Self::default();
            access_control::Internal::_init_with_admin(&mut _instance, Some(Self::env().caller()));
            AccessControl::grant_role(&mut _instance, MANAGER, Some(Self::env().caller()))
                .expect("Should grant MANAGER role");
            AccessControl::grant_role(&mut _instance, MINTER, Some(Self::env().caller()))
                .expect("Should grant MINTER role");
            _instance.metadata.name.set(&name);
            _instance.metadata.symbol.set(&symbol);
            _instance.metadata.decimals.set(&decimal);
            _instance
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(
                Some(String::from("DaoToken")),
                Some(String::from("DAO")),
                18,
            )
        }
    }
}
