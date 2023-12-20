#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contribution_pool {

    use dao_token::dao_token::*;
    use ink::storage::Mapping;
    use registration::registration::*;
use ink::prelude::string::String;  

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Listing {
        listing_id: u64,
        github_repo: String,
        current_deposit: u128,
        payable_amount: u64,
    }
    #[ink::trait_definition]
    pub trait ContributionPoolTrait {
        #[ink(message)]
        #[ink(payable)]
        fn list_repo(&mut self, github_repo: String, payable_amount: u64);
        #[ink(message)]
        fn claim(&mut self, repo: String, pr: String);
        #[ink(message)]
        fn finalize_claim(&mut self, login: String, github_issue: String, amount: u128);
    }
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
   #[derive(Default)]
    pub struct ContributionPool {
        /// Stores a single `bool` value on the storage.
       registration_contract: Option<AccountId>, //* */
        token:  Option<AccountId>, //* */
        assigned_req_id: u64,                          //* */
        counter: u64,                                  //* */
        creator_listings: Mapping<AccountId, Listing>, //* */
        github_to_creator_address: Mapping<String, AccountId>, ////* */
        running_claims: Mapping<u64, AccountId>,       //* */
        request_id_to_creator: Mapping<u64, AccountId>, //* */
        request_id_to_pr_url: Mapping<u64, String>,    //* */
        request_id_to_listing_id: Mapping<u64, u64>,   //* */
        used_requests: Mapping<String, bool>,          //* */
                                                       // listing_id_to_creator: Mapping<u64, AccountId>,//
    }

    impl ContributionPoolTrait for ContributionPool {
        #[ink(message)]
        #[ink(payable)]
        fn list_repo(&mut self, github_repo: String, payable_amount: u64) {
            let caller = Self::env().caller();
            // check if caller is registered
            let is_registered =RegistrationContractRef::is_project_creator_registered(&self.registration_contract.unwrap(),caller);
            
 
            assert!(is_registered);
            // check if money is transferred is more than 1000
            let _transferred = self.env().transferred_value();
            assert!(_transferred >= 10000);

            let listing = Listing {
                listing_id: self.counter,
                github_repo: github_repo.clone(),
                current_deposit: _transferred,
                payable_amount: payable_amount,
            };
            self.creator_listings.insert(caller, &listing);
            // self.listing_id_to_creator
            //     .insert(self.counter, &Self::env().caller());
            self.counter += 1;
        }

        #[ink(message)]
        fn claim(&mut self, repo: String, pr: String) {
            let caller = Self::env().caller();
            let listing = self.creator_listings.get(&caller).unwrap();
            // make sure that the remaining amount is more than the requested amount
            assert!(listing.current_deposit >= u128::from(listing.payable_amount));
            let creator_address = RegistrationContractRef::get_address(&self.registration_contract.unwrap(), repo).unwrap();
          
            let request_id = self.assigned_req_id;
            self.assigned_req_id += 1;
            self.running_claims.insert(request_id, &caller);
            self.request_id_to_creator
                .insert(request_id, &creator_address);
            self.request_id_to_pr_url.insert(request_id, &pr);
            self.request_id_to_listing_id
                .insert(request_id, &listing.listing_id);
        }

        /** */
        #[ink(message)]
        fn finalize_claim(&mut self, login: String, github_issue: String, amount: u128) {
            let caller = Self::env().caller();
            let listing = self.creator_listings.get(&caller).unwrap();
            //get address of github handle
            let dev_address = RegistrationContractRef::get_address(&self.registration_contract.unwrap(), login).unwrap();
            // make sure that the remaining amount is more than the requested amount
            assert!(listing.current_deposit >= u128::from(listing.payable_amount));
            // make the github issue as done
            self.used_requests.insert(github_issue, &true);
            // mint token
            // let mut dao_token_ref = DaoTokenRef(self.token.clone());
            // dao_token_ref.mint(dev_address, amount);

            // DaoTokenRef::mint(&self.token, dev_address, amount);            
            // release payment
            self.env().transfer(caller, amount).unwrap()
        }
    }
    impl ContributionPool {
        /// Constructor that initializes the `bool` value to the given `init_value`.
         #[ink(constructor)]
        pub fn new(registeration_contract_id: Option<AccountId>, token_id: Option<AccountId>) -> Self {
            let mut instance =Self {
                registration_contract:registeration_contract_id,
                token: token_id,
                assigned_req_id: 0,
                counter: 0,
                creator_listings: Mapping::new(),
                running_claims: Mapping::new(),
                request_id_to_creator: Mapping::new(),
                request_id_to_listing_id: Mapping::new(),
                request_id_to_pr_url: Mapping::new(),
                used_requests: Mapping::new(),
                github_to_creator_address: Mapping::new(),
                // listing_id_to_creator: Mapping::new(),
            };
           
            instance
        }


    }
}
