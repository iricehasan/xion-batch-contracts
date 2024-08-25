use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const ORACLE_ADDRESS: Item<Addr> = Item::new("oracle_address");

#[cw_serde]
pub struct PriceData {
    pub denom: String,
    pub decimal: u8,
    pub price_id: String,
}

pub const DATA: Map<String, PriceData> = Map::new("data");
