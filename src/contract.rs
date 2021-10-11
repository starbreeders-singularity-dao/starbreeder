#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, InstantiateMsg, MsgBackProject, MsgCreateProject, QueryMsg};
use crate::state::{State, STATE};
use crate::{error::ContractError, msg::Project};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:backer";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        pool: msg.pool.clone(),
        projects: Vec::new(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("pool", msg.pool.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateProject(m) => create_project(deps, m),
        ExecuteMsg::BackProject(m) => back_project(deps, info, m),
    }
}

pub fn create_project(deps: DepsMut, msg: MsgCreateProject) -> Result<Response, ContractError> {
    msg.validate()?;

    let mut state = STATE.load(deps.storage)?;
    let id = (state.projects.len() + 1) as u128;
    state.projects.push(Project {
        id,
        thumbnail: msg.thumbnail,
        title: msg.title,
        description: msg.description,
        legal_contract: msg.legal_contract,
        funding_requested: msg.funding_requested,
        funding_raised: Vec::new(),
        lockup_period: msg.lockup_period,
        denom: msg.denom,
    });
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "create_project")
        .add_attribute("project_id", id.to_string().as_str()))
}

pub fn back_project(
    deps: DepsMut,
    info: MessageInfo,
    msg: MsgBackProject,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let p = state
        .projects
        .get_mut(msg.id as usize)
        .ok_or(ContractError::NotFound {})?;

    let backer_index = p
        .funding_raised
        .iter()
        .position(|backer| info.sender == backer.0);

    match backer_index {
        Some(i) => {
            let backer = p.funding_raised.get(i).unwrap().clone();
            p.funding_raised
                .insert(i, (backer.0, backer.1 + msg.amount))
        }
        None => p.funding_raised.push((info.sender, msg.amount)),
    }

    let denom = p.denom.clone();
    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_message(BankMsg::Send {
        to_address: state.pool.into_string(),
        amount: vec![Coin {
            denom,
            amount: Uint128::new(msg.amount),
        }],
    }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetProject { id } => to_binary(&get_project(deps, id)?),
    }
}

pub fn get_project(deps: Deps, id: u128) -> StdResult<Project> {
    let state = STATE.load(deps.storage)?;
    let p = state
        .projects
        .get(id as usize)
        .ok_or_else(|| StdError::not_found(format!("project with id {} was not found", id)))?;

    Ok(p.clone())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies(&[]);

//         let msg = InstantiateMsg { pool: "random-addr" };
//         let info = mock_info("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetProject {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let unauth_info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//         match res {
//             Err(ContractError::Unauthorized {}) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_info = mock_info("creator", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
