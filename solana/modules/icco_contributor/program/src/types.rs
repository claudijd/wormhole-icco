use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use solitaire::{
    pack_type,
    processors::seeded::{AccountOwner, Owned},
};
use spl_token::state::{Account, Mint};
//use spl_token_metadata::state::Metadata;

pub type Address = [u8; 32];
pub type ChainID = u16;

/// icco contributor contract configuration Data.
#[derive(Default, Clone, Copy, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Config {
    pub wormhole_bridge: Pubkey,
    pub icco_conductor: Pubkey,
}

impl Owned for Config {
    fn owner(&self) -> AccountOwner {
        AccountOwner::This
    }
}

/// icco sale state. Writeable in init, seal, abort.
#[derive(Default, Clone, Copy, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct SaleState {
    pub is_sealed: u8,
    pub is_aborted: u8,
}

impl Owned for SaleState {
    fn owner(&self) -> AccountOwner {
        AccountOwner::This
    }
}

/// Chain + AccountPubkey.
#[derive(Default, Clone, Copy, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct EndpointRegistration {
    pub chain: ChainID,
    pub contract: Address,
}

impl Owned for EndpointRegistration {
    fn owner(&self) -> AccountOwner {
        AccountOwner::This
    }
}

/*
#[derive(Default, Clone, Copy, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct WrappedMeta {
    pub chain: ChainID,
    pub token_address: Address,
    pub original_decimals: u8,
}

impl Owned for WrappedMeta {
    fn owner(&self) -> AccountOwner {
        AccountOwner::This
    }
}
*/
pack_type!(SplMint, Mint, AccountOwner::Other(spl_token::id()));
pack_type!(SplAccount, Account, AccountOwner::Other(spl_token::id()));
