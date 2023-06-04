use scrypto::prelude::*;

#[blueprint]
mod bootstrap {
    /// This is a bootstrap struct which creates all of the resources which we need to use to test the NFT marketplace.
    struct Bootstrap {}

    impl Bootstrap {
        /// Creates a number of NFT collections used for testing of the NFT marketplace blueprints.
        pub fn bootstrap() -> Vec<Bucket> {
            // Creating the resources used for our non-fungible tokens
            let arachnids: Bucket = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Bored Derivative Junk Club")
                .metadata(
                    "description",
                    "I googled random images and found this, now its an NFT!",
                )
                .metadata("symbol", "ARAC")
                .metadata("cover_image_url", "https://m.media-amazon.com/images/I/71hwEIYs+ZL._AC_UF894,1000_QL80_.jpg")
                .mint_initial_supply([
                    Arachnid {
                        name: "#0001".to_string(),
                        description: "Conceptual find number one".to_string(),
                        key_image_url: "https://cdn.shopify.com/s/files/1/0072/4877/3175/products/cool-dude-cat-with-sunglasses.jpg?v=1569364813".to_string(),
                    },
                    Arachnid {
                        name: "#0002".to_string(),
                        description: "Conceptual find number two".to_string(),
                        key_image_url: "https://images.template.net/wp-content/uploads/2016/04/26122303/Cool-Lion-Colorful-Art.jpg".to_string(),
                    },
                    Arachnid {
                        name: "#0003".to_string(),
                        description: "Conceptual find number three".to_string(),
                        key_image_url: "https://archive.smashing.media/assets/344dbf88-fdf9-42bb-adb4-46f01eedd629/4eb74c27-bc58-4cd2-8cfb-f21bf21cc829/paulgreen.jpg".to_string(),
                    },
                ]);

            let space: Bucket = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Pixel Arachnids")
                .metadata(
                    "description",
                    "Mamajam space time, nuff said.",
                )
                .metadata("symbol", "MMJAM")
                .metadata("cover_image_url", "https://www.shutterstock.com/shutterstock/photos/1182224776/display_1500/stock-vector-vector-pixel-art-scorpion-isolated-cartoon-1182224776.jpg")
                .mint_initial_supply([
                    Phone {
                        name: "#0001".to_string(),
                        description: "Mom's".to_string(),
                        key_image_url: "https://pbs.twimg.com/media/CY4LwAcUsAAIh1y.png".to_string()
                    },
                    Phone {
                        name: "#0002".to_string(),
                        description: "Expensive water".to_string(),
                        key_image_url: "https://static.vecteezy.com/system/resources/previews/023/685/236/original/pixel-art-scorpion-scorpion-insect-pixelated-design-for-logo-web-mobile-app-badges-and-patches-video-game-sprite-8-bit-isolated-illustration-vector.jpg".to_string()
                    },
                    Phone {
                        name: "Real Scorpian".to_string(),
                        description: "Mom's".to_string(),
                        key_image_url: "https://pixelartmaker-data-78746291193.nyc3.digitaloceanspaces.com/image/777b815bb424100.png".to_string()
                    },
                    Phone {
                        name: "Real Scorpian".to_string(),
                        description: "Mom's".to_string(),
                        key_image_url: "https://pixelartmaker-data-78746291193.nyc3.digitaloceanspaces.com/image/777b815bb424100.png".to_string()
                    },
                ]);

            let laptops: Bucket = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Lord Buckethead of Babylon")
                .metadata("description", "Subbuteo, just in the knick of time")
                .metadata("symbol", "SUBT")
                .metadata("cover_image_url", "https://static.independent.co.uk/s3fs-public/thumbnails/image/2019/11/15/16/Count-Binface.jpg")
                .mint_initial_supply([
                    Laptop {
                        name: "Former PM and Buckethead".to_string(),
                        description: "Gunna catch that?".to_string(),
                        key_image_url: "https://i.cbc.ca/1.4153989.1558990177!/fileImage/httpImage/image.jpg_gen/derivatives/16x9_620/lord-buckethead.jpg".to_string(),
                    },
                    Laptop {
                        name: "Dab on your political enemies".to_string(),
                        description: "Gunna eat that?".to_string(),
                        key_image_url: "https://www.thesun.co.uk/wp-content/uploads/2019/11/NINTCHDBPICT000330131203.jpg?strip=all&quality=100&w=1080&h=1080&crop=1".to_string(),
                    }]);
            // With all of the NFTs created, we can now return the buckets of tokens
            return vec![arachnids, space, laptops];
        }
    }
}

#[derive(NonFungibleData, ScryptoSbor)]
struct Arachnid {
    name: String,
    description: String,
    key_image_url: String,
}

#[derive(NonFungibleData, ScryptoSbor)]
struct Phone {
    name: String,
    description: String,
    key_image_url: String,
}

#[derive(NonFungibleData, ScryptoSbor)]
struct Laptop {
    name: String,
    description: String,
    key_image_url: String,
}
