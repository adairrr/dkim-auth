use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Uint64};

// re-export the structs from cw_authenticator
pub use cw_authenticator::AuthenticatorSudoMsg as SudoMsg;
use cw_ownable::cw_ownable_execute;
use crate::counter::params::CounterParams;

#[cw_serde]
pub struct InstantiateMsg {
    // pub params: CounterParams,
}

#[cw_ownable_execute]
#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    Count {},
    Setup {
        params: CounterParams,
    },
    Execute {
        msgs: Vec<CosmosMsg>
    }
}

#[cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum QueryMsg {
    #[returns(CounterResponse)]
    Counter {
        account: String,
        authenticator_id: String,
    },
}

#[cw_serde]
pub struct CounterResponse {
    pub count: Uint64,
}
