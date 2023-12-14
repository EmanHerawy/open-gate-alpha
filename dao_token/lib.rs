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
            Self::new(Some(String::from("DaoToken")), Some(String::from("DAO")), 18)
        }
    }
}
