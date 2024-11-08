use wasm_bindgen::prelude::*;
use zkwasm_rest_abi::*;
pub mod config;
pub mod event;
pub mod gameplay;
pub mod random;
pub mod settlement;
pub mod state;

use crate::config::Config;
use crate::state::{State, Transaction};
zkwasm_rest_abi::create_zkwasm_apis!(Transaction, State, Config);
