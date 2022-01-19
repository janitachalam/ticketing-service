use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub type Extension = Option<Metadata>;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Metadata {
    pub owner: Addr,
    pub show_name: String,
    pub show_date: String,
}

pub const STATE: Item<Metadata> = Item::new("state");
