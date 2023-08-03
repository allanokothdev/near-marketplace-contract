use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, PanicOnDefault, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]

pub struct Marketplace {
    products: UnorderedMap<String, String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, PanicOnDefault)]
pub struct Product {
    id: String,
    name: String,
    description: String,
    image: String,
    location: String,
    price: String,
    owner: AccountId,
    sold: u32
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    id: String,
    name: String,
    description: String,
    image: String,
    location: String,
    price: String
}

#[near_bindgen]
impl Marketplace {
    //  Marketplace methods implementation

    #[init]
    pub fn init() -> Self {
        Self {
            products: UnorderedMap::new(b"product".to_vec()),
        }
    }

    pub fn set_product(&mut self, id: String, product_name: String) {
        self.products.insert(&id, &product_name);
    }

    pub fn get_product(&self, id: &String) -> Option<String> {
        self.products.get(id)
    }
}
