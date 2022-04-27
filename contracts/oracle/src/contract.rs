#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, PriceResponse, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:oracle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        price: msg.price,
        owner: info.sender.clone(),
    };
    if msg.price < 0u64 {
        return Err(ContractError::PriceInstantiationError {});
    }


    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("price", msg.price.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdatePrice { price } => try_update_price(deps, info, price),
    }
    //TODO: execute try_update_price

    // Ok(Response::new())
}

pub fn try_update_price(
    deps: DepsMut,
    info: MessageInfo,
    price: u64,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        if price < 0u64 {
            return Err(ContractError::PriceInstantiationError {});
        }
        state.price = price;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "update"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryPrice {} => to_binary(&query_price(deps)?),
    }
}

fn query_price(deps: Deps) -> StdResult<PriceResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(PriceResponse { price: state.price })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { price: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryPrice {}).unwrap();
        let value: PriceResponse = from_binary(&res).unwrap();
        // assert_eq!(res, Err(StdError::generic_err("not implemented")));
        assert_eq!(17, value.price);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { price: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryPrice {}).unwrap();
        let value: PriceResponse = from_binary(&res).unwrap();
        // assert_eq!(res, Err(StdError::generic_err("not implemented")));
        assert_eq!(17, value.price);

        let info = mock_info("creator", &coins(1000, "earth"));
        let res = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::UpdatePrice { price: 20 },
        );
        match res {
            Err(ContractError::Unauthorized {}) => {
                panic!("Must return unauthorized error")
            }
            _ => {}
        }

        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryPrice {}).unwrap();
        let value: PriceResponse = from_binary(&res).unwrap();
        assert_eq!(20, value.price);
    }
}
