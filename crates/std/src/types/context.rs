use {
    crate::{Addr, Storage},
    serde::{Deserialize, Serialize},
};

/// The context passed by the host to the Wasm module whenever an entry point is
/// called. The module then converts this to Instantiate/Execute/Query or other
/// contexts for easy usage by the contract programmer.
///
/// Some fields may be optional depending on which entry point is called.
/// For example, for queries there is no sender, because queries are not part of
/// a transaction.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Context {
    pub block_height:    u64,
    pub block_timestamp: u64,
    pub sender:          Option<Addr>,
}

pub struct InstantiateCtx<'a> {
    pub store:           &'a mut dyn Storage,
    pub block_height:    u64,
    pub block_timestamp: u64,
    pub sender:          Addr,
}

pub struct ExecuteCtx<'a> {
    pub store:           &'a mut dyn Storage,
    pub block_height:    u64,
    pub block_timestamp: u64,
    pub sender:          Addr,
}

pub struct QueryCtx<'a> {
    pub store:           &'a dyn Storage,
    pub block_height:    u64,
    pub block_timestamp: u64,
}
