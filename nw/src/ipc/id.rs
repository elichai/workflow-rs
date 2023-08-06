//!
//! IPC message identifiers. Provides [`Id64`]
//! and allows for a custom construction of IPC
//! message ids using the [`Generator`] trait.
//!

use crate::ipc::imports::*;

/// Trait representing RPC message `Id` trait constraints
pub trait IdT:
    Generator
    + Debug
    + Clone
    + Eq
    + Hash
    + BorshSerialize
    + BorshDeserialize
    + Serialize
    + DeserializeOwned
    + Send
    + Sync
    + 'static
{
}
impl<T> IdT for T where
    T: Generator
        + Debug
        + Clone
        + Eq
        + Hash
        + BorshSerialize
        + BorshDeserialize
        + Serialize
        + DeserializeOwned
        + Send
        + Sync
        + 'static
{
}

/// `Id` generation trait. This is typically meant to be a random number
/// generator for a cusom message `Id`, but you can also define it to use
/// a sequential generation.
pub trait Generator {
    fn generate() -> Self;
}

/// IPC message id represented by a `u64` type
#[derive(
    Debug, Clone, Eq, PartialEq, Hash, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
pub struct Id64(u64);

impl Generator for Id64 {
    fn generate() -> Self {
        Id64(rand::random())
    }
}
