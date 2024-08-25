use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint256};

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub cw721_code_id: u64,
    pub name: String,
    pub symbol: String,
    pub payment_denom: String,
    pub payment_price: u128,
    pub oracle_helper_addr: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    MintToken {
        extension: Option<Metadata>,
        token_uri: Option<String>,
    },
    UpdateNftPrice {
        new_price: u128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Addr)]
    Token {},
    #[returns(Uint256)]
    NftPrice {},
}

#[cw_serde]
pub struct MigrateMsg {}
