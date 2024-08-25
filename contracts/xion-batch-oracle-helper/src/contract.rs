#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError,
    StdResult,
};
use cw2::set_contract_version;
use pyth_sdk_cw::{query_price_feed, PriceFeedResponse, PriceIdentifier};

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, PriceResponse, QueryMsg};
use crate::state::{PriceData, ADMIN, DATA, ORACLE_ADDRESS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:xion_batch_oracle_helper";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ADMIN.save(deps.storage, &info.sender)?;

    let oracle_address = deps.api.addr_validate(&msg.oracle_address)?;
    ORACLE_ADDRESS.save(deps.storage, &oracle_address)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOracleAddress { oracle_address } => {
            check_admin(&deps, &info)?;
            let oracle_address = deps.api.addr_validate(&oracle_address)?;
            ORACLE_ADDRESS.save(deps.storage, &oracle_address)?;
            Ok(Response::new())
        }
        ExecuteMsg::SetData { data } => {
            check_admin(&deps, &info)?;
            DATA.save(deps.storage, data.denom.clone(), &data)?;
            Ok(Response::new())
        }
        ExecuteMsg::SetDataBatch { data } => {
            check_admin(&deps, &info)?;
            for item in data.iter() {
                DATA.save(deps.storage, item.denom.clone(), item)?;
            }
            Ok(Response::new())
        }
        ExecuteMsg::RemoveData { price_denom } => {
            check_admin(&deps, &info)?;
            DATA.remove(deps.storage, price_denom);
            Ok(Response::new())
        }
    }
}

fn check_admin(deps: &DepsMut, info: &MessageInfo) -> Result<(), ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => {
            let admin = ADMIN.load(deps.storage)?;
            let oracle_address = ORACLE_ADDRESS.load(deps.storage)?;
            let res = ConfigResponse {
                admin: admin.to_string(),
                oracle_address: oracle_address.to_string(),
            };
            Ok(to_json_binary(&res)?)
        }
        QueryMsg::AllDenoms {} => {
            let denoms = DATA
                .range(deps.storage, None, None, Order::Ascending)
                .map(|item| item.unwrap().0)
                .collect::<Vec<String>>();
            Ok(to_json_binary(&denoms)?)
        }
        QueryMsg::PriceId { denom } => {
            let data = DATA.load(deps.storage, denom)?;
            Ok(to_json_binary(&data.price_id)?)
        }
        QueryMsg::Price { denom } => {
            let oracle_address = ORACLE_ADDRESS.load(deps.storage)?;
            Ok(to_json_binary(&get_price(
                deps,
                env,
                oracle_address,
                denom,
            )?)?)
        }
        QueryMsg::Prices {} => {
            let oracle_address = ORACLE_ADDRESS.load(deps.storage)?;
            let price_data = DATA
                .range(deps.storage, None, None, Order::Ascending)
                .map(|item| {
                    let (_, price_data) = item.unwrap();
                    price_data
                })
                .collect::<Vec<PriceData>>();
            let mut prices = vec![];
            for price in price_data {
                let res = get_price(
                    deps,
                    env.clone(),
                    oracle_address.clone(),
                    price.denom.clone(),
                )?;
                prices.push(res);
            }
            Ok(to_json_binary(&prices)?)
        }
        QueryMsg::CheckDenom { denom } => {
            let res = DATA.may_load(deps.storage, denom)?;
            Ok(to_json_binary(&res.is_some())?)
        }
    }
}

fn get_price(
    deps: Deps,
    env: Env,
    oracle_address: Addr,
    denom: String,
) -> Result<PriceResponse, StdError> {
    let data = DATA.load(deps.storage, denom.clone())?;

    let price_feed_response: PriceFeedResponse = query_price_feed(
        &deps.querier,
        Addr::unchecked(oracle_address),
        PriceIdentifier::from_hex(data.price_id).unwrap(),
    )?;
    let price_feed = price_feed_response.price_feed;
    let current_price = price_feed
        .get_price_no_older_than(env.block.time.seconds() as i64, 60)
        .ok_or_else(|| StdError::not_found("Current price is not available"))?;

    Ok(PriceResponse {
        denom,
        price: current_price.price,
        decimal: data.decimal,
    })
}
