use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, PanicOnDefault, AccountId, Promise};
use near_sdk::serde::{ Serialize, Deserialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]

pub struct Marketplace {
    listed_products: UnorderedMap<String, Product>,
}

#[near_bindgen]
impl Marketplace {
    //  Marketplace methods implementation

    #[init]
    pub fn init() -> Self {
        Self {
            listed_products: UnorderedMap::new(b"listed_products".to_vec()),
        }
    }

    pub fn set_product(&mut self, payload: Payload) {
        let product = Product::from_payload(payload);
        self.listed_products.insert(&product.id, &product);
    }

    pub fn get_product(&self, id: &String) -> Option<Product> {
        self.listed_products.get(id)
    }

    pub fn get_products(&self) -> Vec<Product> {
        self.listed_products.values_as_vector().to_vec()
    }

    #[payable]
    pub fn buy_product(&mut self, product_id: &String) {
        match self.listed_products.get(product_id) {
            Some(ref mut product) => {
                let price = product.price.parse().unwrap();
                assert_eq! (env::attached_deposit(), price, "attached deposit should be equal to the price of the product");
                let owner = &product.owner.as_str();
                Promise::new(owner.parse().unwrap()).transfer(price);
                product.increment_sold_amount();
                self.listed_products.insert(&product_id, &product);
            },
            _ => {
                env::panic_str("product not found");
            }
        }
    }
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
impl Product {
    
    pub fn from_payload(payload: Payload) -> Self {
        Self { id: payload.id, name: payload.name, description: payload.description, image: payload.image, location: payload.location, price: payload.price, owner: env::signer_account_id(), sold: 0 }
    }

    pub fn increment_sold_amount(&mut self) {
        self.sold = self.sold + 1;
    }
}
