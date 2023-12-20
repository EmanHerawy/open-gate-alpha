#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable)]
#[openbrush::contract]
pub mod registration {
    use openbrush::*;
use openbrush::traits::String;   
 use dao_token::dao_token::*;
    use ink::codegen::StaticEnv;
    use ink::storage::Mapping;
    use openbrush::{modifiers, traits::Storage};
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
   #[derive(Default, Storage)]
    pub struct Registration {
        /// Stores a single `bool` value on the storage.
        dao_token: Option<AccountId>,
        #[storage_field]
        ownable: ownable::Data,
        treasury_wallet: Option<AccountId>,
        github_to_address: Mapping<String, AccountId>,
        address_to_github: Mapping<AccountId, String>,
        creator_registrated_time: Mapping<AccountId, u64>,
    }
    #[openbrush::trait_definition]
    pub trait RegistrationTrait {
        #[ink(message)]
        fn join_as_contributor(&mut self, github: String);

        #[ink(message)]
        fn join_as_open_source_project_creator(&mut self, github: String);
        #[ink(message)]
        fn get_github(&self, address: AccountId) -> String;
        #[ink(message)]
        fn get_address(&self, github: String) -> Option<AccountId>;
        #[ink(message)]
        fn get_registrated_time(&self, address: AccountId) -> u64;
        #[ink(message)]
        fn get_treasury_wallet(&self) -> Option<AccountId>;
        // #[ink(message)]
        //  fn set_treasury_wallet(&mut self, wallet: AccountId);

        #[ink(message)]
        fn is_project_creator_registered(&self, address: AccountId) -> bool;
    }

    #[openbrush::wrapper]
pub type RegistrationContractRef = dyn RegistrationTrait + Ownable;

    impl RegistrationTrait for Registration {
        #[ink(message)]
        fn join_as_contributor(&mut self, github: String) {
            let caller = Self::env().caller();
            self.github_to_address.insert(github.clone(), &caller);
            self.address_to_github.insert(caller, &github);
            self.creator_registrated_time
                .insert(caller, &Self::env().block_timestamp());
        }

        #[ink(message)]
        fn join_as_open_source_project_creator(&mut self, github: String) {
            let caller = Self::env().caller();
            // check that the OS has enough tokens
            let balance = DaoRef::balance_of(&self.dao_token.unwrap(),caller);
            if balance < 1000 {
                panic!("Not enough tokens to register as a project creator");
            }
            self.github_to_address.insert(github.clone(), &caller);
            self.address_to_github.insert(caller, &github);
            self.creator_registrated_time
                .insert(caller, &Self::env().block_timestamp());
        }
        #[ink(message)]
        fn get_github(&self, address: AccountId) -> String {
            self.address_to_github.get(&address).unwrap_or_default()
        }
        #[ink(message)]
        fn get_address(&self, github: String) -> Option<AccountId> {
            self.github_to_address.get(&github)
        }
        #[ink(message)]
        fn get_registrated_time(&self, address: AccountId) -> u64 {
            self.creator_registrated_time
                .get(&address)
                .unwrap_or_default()
        }
        #[ink(message)]
        fn get_treasury_wallet(&self) -> Option<AccountId> {
            self.treasury_wallet
        }

        #[ink(message)]
        fn is_project_creator_registered(&self, address: AccountId) -> bool {
            self.address_to_github.get(&address).is_some()
        }
    }
    impl Registration {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(wallet: Option<AccountId>, token: Option<AccountId>) -> Self {
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
           instance.dao_token = token;
            instance.treasury_wallet = wallet;
            instance
        }

 
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        // We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let registration = Registration::default();
        //     assert_eq!(registration.get(), false);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut registration = Registration::new(false);
        //     assert_eq!(registration.get(), false);
        //     registration.flip();
        //     assert_eq!(registration.get(), true);
        // }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = RegistrationRef::default();

            // When
            let contract_account_id = client
                .instantiate("registration", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<RegistrationRef>(contract_account_id.clone())
                .call(|registration| registration.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = RegistrationRef::new(false);
            let contract_account_id = client
                .instantiate("registration", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<RegistrationRef>(contract_account_id.clone())
                .call(|registration| registration.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<RegistrationRef>(contract_account_id.clone())
                .call(|registration| registration.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<RegistrationRef>(contract_account_id.clone())
                .call(|registration| registration.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
