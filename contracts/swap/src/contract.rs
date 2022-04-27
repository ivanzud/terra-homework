#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Coin, Addr,attr, BankMsg, Uint128, Binary, CosmosMsg, WasmMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
    StdResult, QueryRequest, WasmQuery
};
use cw20:: {Cw20ExecuteMsg};
use cw2::set_contract_version;
use shared::oracle::{PriceResponse, QueryMsg as oracle_query};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use terraswap::querier::{query_balance};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state: State = State{
        owner: info.sender,
        token_address: msg.token_address,
        oracle_address: msg.oracle_address,
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::Buy {} => try_buy(deps, info),
        ExecuteMsg::Withdraw {amount} => try_withdraw(deps, env, info, amount),
    }
    // Err(ContractError::NotImplemented {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    // TODO
    Ok(Response::default())
}

pub fn try_buy(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state: State = STATE.load(deps.storage)?;

    let mut price: u64 = query_oracle(deps.as_ref(), state.oracle_address)?;

    let coin_amount: Uint128 = info
    .funds
    .iter()
    .find(|c| c.denom == "uluna")
    .map(|c| Uint128::from(c.amount))
    .unwrap_or_else(Uint128::zero);

    if coin_amount < Uint128::from(0_000_000 as u128) {
        return Err(ContractError::NotEnoughFunds {
        });
    }
    let amount_of_token: Uint128 = Uint128::from(1u64).multiply_ratio(coin_amount, Uint128::from(price));

    let mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: state.token_address.into(),
        funds: vec![],
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.into(),
            amount: amount_of_token,
        })?,
    });

    let res = Response::new()
        .add_attributes(vec![attr("action", "buy_lemons")])
        .add_message(mint_msg);

    Ok(res)
}

pub fn try_withdraw(deps: DepsMut, env: Env, info: MessageInfo, amount: i32) -> Result<Response, ContractError> {
    // this method withdraws the entire balance from smart contract
    // you might want to restrict how much UST can be withdrawn

    let state: State = STATE.load(deps.storage)?;

    if state.owner != info.sender {
        return Err(ContractError::Unauthorized{});
    }
    
    if amount <= 0i32 {
        return Err(ContractError::InvalidQuantity{});
    }

    let balance = query_balance(&deps.querier, env.contract.address.clone(), "uluna".to_string())?;

    if balance <  Uint128::from(amount as u128) {
        return Err(ContractError::NotEnoughFunds{});
    }
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.clone().into_string(),
        amount: vec![
            Coin {
                denom: "uluna".to_string(),
                amount: balance,
            },
        ],
    });


    Ok(Response::new().add_message(msg))
}

pub fn query_oracle(deps: Deps, oracle_address: Addr) -> StdResult<u64> {
    // load price form the oracle
    let price_response: PriceResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: oracle_address.to_string(),
            msg: to_binary(&oracle_query::QueryPrice {
            })?,
        }))?;

    Ok(price_response.price)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {} => Ok({
            let state: State = STATE.load(deps.storage)?;
            to_binary(&state)?
        }),
        
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn proper_initialization() {

        //TODO
    }
}
