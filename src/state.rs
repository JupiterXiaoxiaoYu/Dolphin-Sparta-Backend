use crate::gameplay::{update_state, PlayerData};
use crate::settlement::SettlementInfo;
use crate::MERKLE_MAP;
use serde::Serialize;
use zkwasm_rest_abi::Player;
use crate::event::QUEUE;
use std::cell::RefCell;


pub type DolphinPlayer = Player<PlayerData>;

#[derive(Serialize)]
pub struct State {
}

impl State {
    pub fn get_state(pkey: Vec<u64>) -> String {
        let player =
            DolphinPlayer::get_from_pid(&DolphinPlayer::pkey_to_pid(&pkey.try_into().unwrap()));
        serde_json::to_string(&player).unwrap()
    }

    pub fn rand_seed() -> u64 {
        0
    }

    pub fn store(&self) {
        QUEUE.0.borrow_mut().store();
    }

    pub fn initialize() {
        QUEUE.0.borrow_mut().fetch();
    }

    pub fn new() -> Self {
        State {}
    }

    pub fn snapshot() -> String {
        let state = unsafe { &STATE };
        serde_json::to_string(&state).unwrap()
    }

    pub fn preempt() -> bool {
        return false;
    }

    pub fn flush_settlement() -> Vec<u8> {
        let data = SettlementInfo::flush_settlement();
        unsafe { STATE.store() };
        data
    }
}

pub static mut STATE: State = State {};

pub struct Transaction {
    pub command: u64,
    pub nonce: u64,
    pub data: Vec<u64>,
}

// 简化命令常量
const CMD_TICK: u64 = 0;
const INSTALL_PLAYER: u64 = 1;

// 简化错误常量
const ERROR_PLAYER_ALREADY_EXIST: u32 = 1;
const ERROR_PLAYER_NOT_EXIST: u32 = 2;
const ERROR_INVALID_COMMAND: u32 = 3;

impl Transaction {
    pub fn decode_error(e: u32) -> &'static str {
        match e {
            ERROR_PLAYER_NOT_EXIST => "PlayerNotExist",
            ERROR_PLAYER_ALREADY_EXIST => "PlayerAlreadyExist",
            ERROR_INVALID_COMMAND => "InvalidCommand",
            _ => "Unknown",
        }
    }
    pub fn decode(params: [u64; 4]) -> Self {
        let command = params[0] & 0xff;
        let nonce = params[0] >> 16;
        let data = vec![params[1], params[2], params[3]]; // pkey[0], pkey[1], amount
        Transaction { command,nonce, data }
    }
    pub fn install_player(&self, pkey: &[u64; 4]) -> u32 {
        let pid = DolphinPlayer::pkey_to_pid(pkey);
        let player = DolphinPlayer::get_from_pid(&pid);
        match player {
            Some(_) => ERROR_PLAYER_ALREADY_EXIST,
            None => {
                let player = DolphinPlayer::new_from_pid(pid);
                player.store();
                0
            }
        }
    }

    pub fn process(&self, pkey: &[u64; 4], _rand: &[u64; 4]) -> u32 {
        let cmd_category = self.command >> 4;
        let b = if cmd_category == 1 {
            let pid = DolphinPlayer::pkey_to_pid(pkey);
            let mut player = DolphinPlayer::get_from_pid(&pid);
            
            if let Some(mut player) = player {
                update_state(self.command & 0xf, &mut player.data, self.data[0], _rand[0])
                    .map_or(ERROR_INVALID_COMMAND, |_| 0);
                player.check_and_inc_nonce(self.nonce);
                player.store();
                0
            } else {
                ERROR_PLAYER_NOT_EXIST
            }
        } else {
            match self.command {
                CMD_TICK => QUEUE.0.borrow_mut().tick(),
                INSTALL_PLAYER => self.install_player(pkey),
                _ => 0,
            }
        };
        let kvpair = unsafe { &mut MERKLE_MAP.merkle.root };
        zkwasm_rust_sdk::dbg!("root after process {:?}\n", kvpair);
        b
    }
}
