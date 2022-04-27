#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
    StdResult,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // TODO
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // TODO
    match msg {
        ExecuteMsg::Buy {} => try_buy(deps),
        ExecuteMsg::Withdraw {amount} => try_withdraw(deps, amount),
    }
    // Err(ContractError::NotImplemented {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    // TODO
    Ok(Response::default())
}
  // Attempts to buy Lemons. Caller must pass some Luna with this ExecuteMsg
  // and should receive some Lemons in exchange. This contract keeps the Luna.
  // Internally, this method queries the price from the Oracle contract, then
  // computes how many Lemons to send to the buyer (according to the current
  // price and how much Luna they sent).
  // Fails if this contract doesn't own enough Lemons in the CW20 contract.
  // Public endpoint (anyone can call).

pub fn try_buy(_deps: DepsMut) -> Result<Response, ContractError> {
    

    Ok(Response::new())
}

pub fn try_withdraw(_deps: DepsMut, amount: i32) -> Result<Response, ContractError> {
    // TODO
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // TODO
    Err(StdError::generic_err("Not implemented"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn proper_initialization() {

        //TODO
    }
}
