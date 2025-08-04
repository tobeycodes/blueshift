use {
    num_derive::FromPrimitive,
    pinocchio::program_error::{ProgramError, ToStr},
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PinocchioError {
    #[error("Not the signer")]
    NotSigner,

    #[error("Invalid account owner")]
    InvalidOwner,

    #[error("Invalid account data")]
    InvalidAccountData,

    #[error("Invalid address")]
    InvalidAddress,
}

impl From<PinocchioError> for ProgramError {
    fn from(e: PinocchioError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl TryFrom<u32> for PinocchioError {
    type Error = ProgramError;
    fn try_from(error: u32) -> Result<Self, Self::Error> {
        match error {
            0 => Ok(PinocchioError::NotSigner),
            1 => Ok(PinocchioError::InvalidOwner),
            2 => Ok(PinocchioError::InvalidAccountData),
            3 => Ok(PinocchioError::InvalidAddress),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl ToStr for PinocchioError {
    fn to_str<E>(&self) -> &'static str {
        match self {
            PinocchioError::NotSigner => "Error: Not the signer of the transaction",
            PinocchioError::InvalidOwner => "Error: Invalid account owner",
            PinocchioError::InvalidAccountData => "Error: Invalid account data",
            PinocchioError::InvalidAddress => "Error: Invalid address",
        }
    }
}
