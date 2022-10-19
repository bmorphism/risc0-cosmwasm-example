use cosmwasm_schema::{cw_serde, QueryResponses};

use risc0_zkvm_verify::zkvm::{MethodID, Receipt};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    VerifyReceipt { receipt: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
