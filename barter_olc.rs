use scrypto::prelude::*;

// OVERVIEW OF BLUEPRINT

// This blueprint allows the exchange of assets between users in a trustless and scrypto-orientate fashion - commonly
// known as an escrow or peer-to-peer trading service. This blueprint does not reinvent the wheel, aiming solely to
// provide a quick and customisable trading utility.

// USER FLOW

// Alice creates a trade proposal by instantiating a component from this Blueprint. When instantiating the component
// she deposits the assets she wants to trade into a hashmap of vaults in the component and sends a list of 
// of resource addresses and quanitities for the assets she would like to receive in return. She'll also receive an NFT badge
// containing details of here trade proposal - this protects certain methods at a system level. 

// Alice can then link her trading partner Bob to this trade with a short code generated on the front-end. 
// This code is linked to the new component address that's been created on the TROVE back-end server.

// Bob can see what Alice has offered and what he needs to send in return. If he wants to take the trade,
// he must deposit ALL of the assets specified in the trade proposal as they are compared against the list 
// Alice provided earlier. If all the assets are present, they are deposited into another hashmap of vaults
// within the component and the internal state of the component updates to allow him to withdraw Alice's assets.
// Through the front-end and transaction manifest, this is all one transaction for Bob.

// Alice can then withdraw the assets that Bob has deposited.

// Alice also has a method to cancel the trade if it has not yet been completed in order to retrieve her
// assets, if needed. 

//=================================================================
// To Do list:
// 1. Integrate a fee collector - whether by v.0.10's royalty system or otherwise.
// 2. Split out component instatiations for customisation - i.e. trade with a time limit or adding permitted accounts
// 3. Add potential use of event emitters for notifying changes on the front-end
// 4. Test and limit size of trade due to hashmap limits - avoid stuck assets in the component
// 5. Further checks for edge cases, possibilities of stuck resources...
// 6. Optimisations for fees - i.e. badge minting and loop methods used


// A larger "to do", is to try to enforce NFT royalties as part of the system. The intention is not to create
// a solution for avoiding creator royalties - however this will depend heavily upon the standard
// that is adopted by the community, i.e. clearing house, transient tokens, etc. 


//=================================================================

#[blueprint]

mod barter {

    struct Barter {
        a_nft_vaults: HashMap<ResourceAddress, Vault>,
        a_vault_key: ResourceAddress,
        a_vault_key_id: NonFungibleLocalId,
        b_nft_vaults: HashMap<ResourceAddress, Vault>,
        badge_sweeper: Vault,
        receipt_addr: ResourceAddress,
        receipt_gaddr: NonFungibleLocalId,
        escrow_receipt: Vault,
        expected_nfts: Vec<NonFungibleGlobalId>,
        expected_tokens: HashMap<ResourceAddress, Decimal>,
        tokens_validated: bool,
        nfts_validated: bool,
    }

    impl Barter {
        pub fn lets_barter_partner_permitted_clock(
            custom_trade_name: String,
            a_nft_buckets: Vec<Bucket>,
            a_token_buckets: Vec<Bucket>,
            b_nft_deposits: Vec<NonFungibleGlobalId>,
            b_token_deposits: HashMap<ResourceAddress, Decimal>,
        ) -> (ComponentAddress, Bucket) {
         

            let mut a_nft_deposits: Vec<NonFungibleGlobalId> = Vec::new();

            for bucket in a_nft_buckets.iter() {
                let fucket: NonFungible<Escroceipt> = bucket.non_fungible();
                let example: NonFungibleGlobalId = fucket.global_id().clone();
                a_nft_deposits.push(example);
            }

            let mut a_token_deposits: HashMap<ResourceAddress, Decimal> = HashMap::new();

            for i in a_token_buckets.iter() {
                a_token_deposits.insert(i.resource_address(), i.amount());
            }

            let my_nfts = a_nft_deposits.clone();
            let my_tokens = a_token_deposits.clone();

            //

            // create log of expected assets to be deposited by User B

            let expected_b_nft_deposits = b_nft_deposits.clone();
            let expected_b_token_deposits = b_token_deposits.clone();

            // clone again

            let nfts_i_expect = b_nft_deposits.clone();
            let tokens_i_expect = b_token_deposits.clone();

            //

            // cut in Scrypto v.10

            let badge_sweeper: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            //

            let key_custom_name = "TROVE proposal: ".to_string() + &custom_trade_name;

            // internally held NFT keeping record of assets deposited and assets required to execute trade

            let escrow_record = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", &key_custom_name)
                .mintable(rule!(require(badge_sweeper.resource_address())), LOCKED)
                .burnable(rule!(require(badge_sweeper.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(badge_sweeper.resource_address())),
                    LOCKED,
                )
                .mint_initial_supply([Escroceipt {
                    a_nft_contents: a_nft_deposits,
                    a_token_contents: a_token_deposits,
                    b_nft_contents: b_nft_deposits,
                    b_token_contents: b_token_deposits,
                    ready_to_claim: false,
                }]);
            //

            // Key provided to User A who instantiates the component

            let a_key = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", &key_custom_name)
                .metadata("description", "Your proposal to a trade on trove.eco")
                .metadata(
                    "key_image_url",
                    "https://cdn.alphawholesale.co.uk/product/images/main/cfws52.jpg",
                )
                .mintable(rule!(require(badge_sweeper.resource_address())), LOCKED)
                .burnable(rule!(require(badge_sweeper.resource_address())), LOCKED)
                .updateable_non_fungible_data(
                    rule!(require(badge_sweeper.resource_address())),
                    LOCKED,
                )
                .mint_initial_supply([Escroceipt {
                    a_nft_contents: my_nfts,
                    a_token_contents: my_tokens,
                    b_nft_contents: nfts_i_expect,
                    b_token_contents: tokens_i_expect,
                    ready_to_claim: false,
                }]);
            

            // Deposit user A assets into hashmap of vaults

            let mut user_a_vaults = HashMap::new();

            for bucket in a_nft_buckets.into_iter() {
                user_a_vaults
                    .entry(bucket.resource_address())
                    .or_insert(Vault::new(bucket.resource_address()))
                    .put(bucket)
            }

            for bucket in a_token_buckets.into_iter() {
                user_a_vaults
                    .entry(bucket.resource_address())
                    .or_insert(Vault::new(bucket.resource_address()))
                    .put(bucket)
            }

            //

            let rules = AccessRulesConfig::new()
                .method("cancel", rule!(require(a_key.resource_address())), LOCKED)
                .method(
                    "claim_my_new_assets",
                    rule!(require(a_key.resource_address())),
                    LOCKED,
                )
                .default(AccessRule::AllowAll, AccessRule::DenyAll);

            let component = Self {
                a_vault_key: a_key.resource_address(),
                a_vault_key_id: a_key.non_fungible_local_id(),
                a_nft_vaults: user_a_vaults,
                b_nft_vaults: HashMap::new(),
                badge_sweeper: Vault::with_bucket(badge_sweeper),
                expected_nfts: expected_b_nft_deposits,
                expected_tokens: expected_b_token_deposits,
                receipt_addr: escrow_record.resource_address(),
                receipt_gaddr: escrow_record.non_fungible_local_id(),
                escrow_receipt: Vault::with_bucket(escrow_record),
                tokens_validated: false,
                nfts_validated: false,
            }
            .instantiate();
            let component_address = component.globalize_with_access_rules(rules);

            // return badge to a
            (component_address, a_key)
        }


// Separated methods for depositing either tokens or NFTs as the details are treated differently.
// This makes it so it's not mandatory to include both NFTs and tokens in a trade.
// Inclusion of each method can be determined on the front-end.

        pub fn b_deposit_nfts(&mut self, b_nft_assets: Vec<Bucket>) {
            let mut nft_record: Vec<NonFungibleGlobalId> = Vec::new();

            for i in &b_nft_assets {
                let nft_resource = i.resource_address();
                let nft_id = i.non_fungible_local_id();
                let nft_global = NonFungibleGlobalId::new(nft_resource, nft_id);
                nft_record.push(nft_global)
            }

            let sorted_record = nft_record.sort();
            let sorted_criteria = self.expected_nfts.sort();

            assert!(
                sorted_record == sorted_criteria,
                "Deposit does not meet A's expectations"
            );

            for bucket in b_nft_assets.into_iter() {
                self.b_nft_vaults
                    .entry(bucket.resource_address())
                    .or_insert(Vault::new(bucket.resource_address()))
                    .put(bucket)
            }

            self.nfts_validated = true
        }

        pub fn b_deposit_tokens(&mut self, b_token_assets: Vec<Bucket>) {
            let token_criteria = self.expected_tokens.clone();
            let mut b_deposit_hm: HashMap<ResourceAddress, Decimal> = HashMap::new();
            for bucket in b_token_assets.iter() {
                b_deposit_hm.insert(bucket.resource_address(), bucket.amount());
            }

            assert!(b_deposit_hm == token_criteria, "Token deposits don't match");

            for bucket in b_token_assets.into_iter() {
                self.b_nft_vaults
                    .entry(bucket.resource_address())
                    .or_insert(Vault::new(bucket.resource_address()))
                    .put(bucket)
            }

            self.tokens_validated = true
        }

        pub fn b_claims_a_assets(&mut self) -> Vec<Bucket> {

            // provide optionality for trades to include either tokens, nfts or both tokens and nfts.

            if !self.expected_nfts.len() > 0 {
                assert!(
                    self.nfts_validated,
                    "Insufficient assets deposited for trade"
                )
            }
            if !self.expected_tokens.is_empty() {
                assert!(
                    self.tokens_validated,
                    "Insufficient assets deposited for trade"
                )
            }

            let a_assets: Vec<ResourceAddress> = self.a_nft_vaults.keys().cloned().collect();

            let mut buckets: Vec<Bucket> = vec![];

            for resource_address in a_assets.into_iter() {
                buckets.push(
                    self.a_nft_vaults
                        .get_mut(&resource_address)
                        .unwrap()
                        .take_all(),
                )
            }

            self.badge_sweeper.authorize(|| {
                borrow_resource_manager!(self.receipt_addr).update_non_fungible_data(
                    &self.receipt_gaddr,
                    "ready_to_claim",
                    true,
                )
            });
            self.badge_sweeper.authorize(|| {
                borrow_resource_manager!(self.a_vault_key).update_non_fungible_data(
                    &self.a_vault_key_id,
                    "ready_to_claim",
                    true,
                )
            });

            return buckets;
        }

        pub fn cancel(&mut self) -> Vec<Bucket> {
            assert!(
                self.b_nft_vaults.is_empty(),
                "Trade has already occured, only able to withdraw new assets"
            );

            let a_assets: Vec<ResourceAddress> = self.a_nft_vaults.keys().cloned().collect();

            let mut buckets: Vec<Bucket> = vec![];

            for resource_address in a_assets.into_iter() {
                buckets.push(
                    self.a_nft_vaults
                        .get_mut(&resource_address)
                        .unwrap()
                        .take_all(),
                )
            }

            return buckets;
        }

        pub fn claim_my_new_assets(&mut self) -> Vec<Bucket> {
            assert!(
                !self.b_nft_vaults.is_empty(),
                "Trade hasn't occured, waiting for trading partner"
            );

            let b_assets: Vec<ResourceAddress> = self.b_nft_vaults.keys().cloned().collect();

            let mut buckets: Vec<Bucket> = vec![];

            for resource_address in b_assets.into_iter() {
                buckets.push(
                    self.b_nft_vaults
                        .get_mut(&resource_address)
                        .unwrap()
                        .take_all(),
                )
            }
            return buckets;
        }

        // after accepted
    }
}

#[derive(NonFungibleData, ScryptoSbor, Debug)]
struct Escroceipt {
    a_nft_contents: Vec<NonFungibleGlobalId>,
    a_token_contents: HashMap<ResourceAddress, Decimal>,
    b_nft_contents: Vec<NonFungibleGlobalId>,
    b_token_contents: HashMap<ResourceAddress, Decimal>,
    #[mutable]
    ready_to_claim: bool,
}
