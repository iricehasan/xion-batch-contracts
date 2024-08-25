use std::collections::HashMap;

use cosmwasm_std::{Deps, DepsMut, MessageInfo, QuerierWrapper, Storage};

use crate::{
    state::{ORACLE_HELPER_ADDR, PAYMENT_DENOM},
    ContractError,
};

use xion_batch_oracle_helper::msg::{PriceResponse, QueryMsg as OracleHelperQueryMsg};

pub fn query_price(
    storage: &dyn Storage,
    querier: QuerierWrapper,
    denom: &str,
) -> Result<PriceResponse, ContractError> {
    let oracle_helper_addr = ORACLE_HELPER_ADDR.load(storage)?;
    let res: PriceResponse = querier.query_wasm_smart(
        oracle_helper_addr,
        &OracleHelperQueryMsg::Price {
            denom: denom.to_string(),
        },
    )?;
    Ok(res)
}

pub fn query_all_prices(deps: &Deps) -> Result<HashMap<String, PriceResponse>, ContractError> {
    let oracle_helper_addr = ORACLE_HELPER_ADDR.load(deps.storage)?;
    let prices: Vec<PriceResponse> = deps
        .querier
        .query_wasm_smart(oracle_helper_addr, &OracleHelperQueryMsg::Prices {})?;
    let mut map: HashMap<String, PriceResponse> = HashMap::new();
    for price_response in prices {
        map.insert(price_response.clone().denom, price_response);
    }
    Ok(map)
}

pub fn query_all_denoms(deps: &DepsMut) -> Result<Vec<String>, ContractError> {
    let oracle_helper_addr = ORACLE_HELPER_ADDR.load(deps.storage)?;
    let denoms: Vec<String> = deps
        .querier
        .query_wasm_smart(oracle_helper_addr, &OracleHelperQueryMsg::AllDenoms {})?;
    Ok(denoms)
}

// TODO: Give the denom in instantiate_msg and save it to a state. Then, load it from the state to check in this function
// add nft price to contract state
pub fn check_funds(deps: &Deps, info: &MessageInfo) -> Result<(), ContractError> {
    if info.funds.len() == 0 {
        return Err(ContractError::MissingFunds {});
    }

    if info.funds.len() > 1 {
        return Err(ContractError::ExtraFunds {});
    }

    let denom = PAYMENT_DENOM.load(deps.storage)?;

    let sent_fund = info.funds.get(0).unwrap();
    if sent_fund.denom != denom {
        return Err(ContractError::InvalidDenom {
            got: sent_fund.denom.clone(),
            expected: denom.to_string(),
        });
    }
    Ok(())
}
