use scrypto::prelude::*;

#[blueprint]
mod bootstrappartner {
    /// This is a bootstrap struct which creates all of the resources which we need to use to test the NFT marketplace.
    struct Bootstrappartner {}

    impl Bootstrappartner {
        /// Creates a number of NFT collections used for testing of the NFT marketplace blueprints.
        pub fn bootstrap_partner() -> Vec<Bucket> {
            // non fungible creation
            let test_fun: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Ociswap")
                .metadata("symbol", "OCI")
                .metadata(
                    "icon_url",
                    "https://pbs.twimg.com/profile_images/1555153509355511809/yiIU25Gw_400x400.jpg",
                )
                .mint_initial_supply(5000);

            // Creating the resources used for our non-fungible tokens

            let arachnids: Bucket = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Olympics 2024 Paris")
                .metadata(
                    "description",
                    "I googled random images and found this, now its an NFT!",
                )
                .metadata("symbol", "OLYM")
                .metadata("cover_image_url", "https://img.olympicchannel.com/images/image/private/t_s_w338/f_auto/primary/gpo3co3bpkqsikyznrns")
                .mint_initial_supply([
                    Arachnid {
                        name: "Men's Street Skateboard Final- 24 July".to_string(),
                        description: "Conceptual find number one".to_string(),
                        key_image_url: "https://static01.nyt.com/images/2021/07/13/sports/00skate-health-1/merlin_188173380_7e396d4d-c2bf-4a83-b44b-4348bf6cc1ec-articleLarge.jpg?quality=75&auto=webp&disable=upscale".to_string(),
                    },
                    Arachnid {
                        name: "Women's Archery Final - 21 July".to_string(),
                        description: "Conceptual find number two".to_string(),
                        key_image_url: "https://img.olympicchannel.com/images/image/private/t_16-9_640/f_auto/v1538355600/primary/o2dn1lo38v9i5o5xnvbd".to_string(),
                    },
                    Arachnid {
                        name: "Track: Men's 100m Final - 29 July".to_string(),
                        description: "Conceptual find number three".to_string(),
                        key_image_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/0/01/DOH20288_100m_final_men_%2848911163412%29.jpg/640px-DOH20288_100m_final_men_%2848911163412%29.jpg".to_string(),
                    },
                ]);

            // With all of the NFTs created, we can now return the buckets of tokens
            return vec![arachnids, test_fun];
        }
    }
}

#[derive(NonFungibleData, ScryptoSbor)]
struct Arachnid {
    name: String,
    description: String,
    key_image_url: String,
}
