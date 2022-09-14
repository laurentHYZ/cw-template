use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address: cosmwasm_std::Addr, //ex juno5678
}

pub const CONFIG: Item<Config> = Item::new("state");
