//! This crate contains code that is shared between the VM host (the ledger) and
//! the guest (wasm code).

mod imports;
mod token;

pub mod tx_prelude {
    pub use anoma_shared::types::{token, *};
    pub use anoma_shared::vm_memory;

    pub use super::imports::tx::*;
    pub use super::token::transfer as token_transfer;
}

pub mod vp_prelude {
    pub use anoma_shared::types::{token, *};
    pub use anoma_shared::vm_memory;

    pub use super::imports::vp::*;
    pub use super::token::vp as token_vp;
}

pub mod matchmaker_prelude {
    pub use anoma_shared::types::*;
    pub use anoma_shared::{vm_memory, *};

    pub use super::imports::matchmaker::*;
}
