use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// pub const ADMIN: Item<Addr> = Item::new("admin");
pub const TOKEN: Item<Addr> = Item::new("token");
pub const TOKEN_COUNT: Item<u64> = Item::new("token_count");
pub const ORACLE_HELPER_ADDR: Item<Addr> = Item::new("oracle_helper_addr");
pub const NFT_PRICE: Item<u128> = Item::new("nft_price");
pub const PAYMENT_DENOM: Item<String> = Item::new("payment_denom");
