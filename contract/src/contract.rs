#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use risc0_zkvm::host::{MethodId, Receipt};
use risc0_zkvm_core::Digest;

use methods::{MULTIPLY_ID, MULTIPLY_PATH};

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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::VerifyReceipt { receipt } => verify_receipt(receipt),
    }
}

pub fn verify_receipt(receipt: String) -> Result<Response, ContractError> {
    let as_bytes = base64::decode(receipt).unwrap();
    let receipt = bincode::deserialize::<Receipt>(&as_bytes).unwrap();

    // Verify that the zero knowledge proof is valid
    if !receipt.verify(MULTIPLY_ID).is_ok() {
        return Err(ContractError::VerificationError {});
    };

    let journal = receipt.get_journal_vec().unwrap();
    println!("{:?}", journal);
    // let digest = risc0_zkvm_serde::from_slice::<Digest>(&journal).unwrap();
    // println!("{:?}", digest);

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::*;

    use methods::{MULTIPLY_ID, MULTIPLY_PATH};
    use risc0_zkvm::host::Prover;
    use risc0_zkvm::serde::{from_slice, to_vec};

    #[test]
    fn test_verify_recipt() {
        // Pick two numbers
        let a: u64 = 7;
        let b: u64 = 191;

        // Multiply them inside the ZKP
        // First, we make the prover, loading the 'multiply' method
        let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();
        // Next we send a & b to the guest
        prover.add_input(to_vec(&a).unwrap().as_slice()).unwrap();
        prover.add_input(to_vec(&b).unwrap().as_slice()).unwrap();
        // Run prover & generate receipt
        let receipt = prover.run().unwrap();
        print_type_of(&receipt);

        // Extract journal of receipt (i.e. output c, where c = a * b)
        let journal: u64 = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
        // Print an assertion
        println!("I know the factors of {}, and I can prove it!", journal);

        // Verify receipt, panic if it's wrong
        receipt.verify(MULTIPLY_ID).unwrap();
        println!("CORRECT!");

        // Here is where one would send 'receipt' over the network...
        // We must encode as a String
        let encoded_receipt = base64::encode(bincode::serialize(&receipt).unwrap());
        // println!("{}", encoded_receipt);

        // Call verify receipt contract method
        let res = verify_receipt(encoded_receipt);
        println!("{:?}", res);
        assert!(res.is_ok());
    }
}

// TODO remove this debug util
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
