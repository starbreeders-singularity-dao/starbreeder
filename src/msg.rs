use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ContractError;

// TODO randomly create pool

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub pool: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateProject(MsgCreateProject),
    BackProject(MsgBackProject),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MsgCreateProject {
    pub thumbnail: String,
    pub title: String,
    pub description: String,
    pub legal_contract: String,
    pub funding_requested: u64,
    pub lockup_period: u64,
    pub denom: String,
}

impl MsgCreateProject {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.thumbnail.chars().count() > 100 {
            return Err(ContractError::InvalidInput(
                "thumbnail too large, should be less than 100 chars".to_string(),
            ));
        }

        if self.title.chars().count() > 75 {
            return Err(ContractError::InvalidInput(
                "title too large, should be less than 75 chars".to_string(),
            ));
        }

        if self.description.chars().count() > 400 {
            return Err(ContractError::InvalidInput(
                "description too large, should be less than 400 chars".to_string(),
            ));
        }

        if self.legal_contract.chars().count() > 100 {
            return Err(ContractError::InvalidInput(
                "legal_contract too large, should be less than 100 chars".to_string(),
            ));
        }

        if self.denom.chars().count() > 100 {
            return Err(ContractError::InvalidInput(
                "denom too large, should be less than 100 chars".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MsgBackProject {
    pub id: u64,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetProject { id: u64 },
}

// TODO store backers in map
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Project {
    pub id: u64,
    pub thumbnail: String,
    pub title: String,
    pub description: String,
    pub legal_contract: String,
    pub funding_requested: u64,
    pub funding_raised: Vec<(Addr, u64)>,
    pub lockup_period: u64,
    pub denom: String,
}
