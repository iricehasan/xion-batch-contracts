use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::PriceData;

#[cw_serde]
pub struct InstantiateMsg {
    pub oracle_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateOracleAddress { oracle_address: String },
    SetData { data: PriceData },
    SetDataBatch { data: Vec<PriceData> },
    RemoveData { price_denom: String },
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub oracle_address: String,
}

#[cw_serde]
pub struct PriceResponse {
    pub denom: String,
    pub decimal: u8,
    pub price: i64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(Vec<String>)]
    AllDenoms {},
    #[returns(Vec<String>)]
    PriceId { denom: String },
    #[returns(PriceResponse)]
    Price { denom: String },
    #[returns(Vec<PriceResponse>)]
    Prices {},
    #[returns(bool)]
    CheckDenom { denom: String },
}
