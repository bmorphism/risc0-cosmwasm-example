use cosmwasm_std::StdError;
use thiserror::Error;

struct NgmiError{
    
}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    // #[error("Verification error")]
    // VerificationError {},
}

// impl From<risc0_zkvm_verify::zkp::verify::VerificationError> for ContractError {
//     fn from(
//         msg: risc0_zkvm_verify::zkp::verify::VerificationError,
//     ) -> ContractError::VerificationError {
//     }
// }
