#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use risc0_zkvm_core::Digest;
use risc0_zkvm_verify::zkvm::{MethodID, Receipt};

use methods;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:risc0-cosmwasm-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::VerifyReceipt { receipt } => verify_receipt(deps, env, info, receipt),
    }
}

pub fn verify_receipt(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    receipt: String,
) -> Result<Response, ContractError> {
    let method_id = MethodID::try_from(methods::MULTIPLY_ID).unwrap();
    let as_bytes = base64::decode(receipt).unwrap();
    let receipt = bincode::deserialize::<Receipt>(&as_bytes).unwrap();

    // Verify that the zero knowledge proof is valid
    if !receipt.verify(&method_id).is_ok() {
        return Err(ContractError::VerificationError {});
    };

    let journal = receipt.get_journal_u32();
    let digest = risc0_zkvm_serde::from_slice::<Digest>(&journal).unwrap();

    println!("{:?}", digest);

    unimplemented!();
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::*;

    #[test]
    fn test_verify_recipt() {
        let mock_deps = mock_dependencies();
        let mock_env = mock_env();
        let mock_info = mock_info("meow", []);
    }
}
