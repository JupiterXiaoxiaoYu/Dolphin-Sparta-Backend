use crate::gameplay::PlayerData;
use crate::MERKLE_MAP;
use zkwasm_rest_abi::Player;
use serde::Serialize;
use crate::settlement::SettlementInfo;

pub type DolphinPlayer = Player<PlayerData>;

#[derive (Serialize)]
pub struct State {}

impl State {
    pub fn get_state(pkey: Vec<u64>) -> String {
        let player = DolphinPlayer::get_from_pid(&DolphinPlayer::pkey_to_pid(&pkey.try_into().unwrap()));
        serde_json::to_string(&player).unwrap()
    }

    pub fn rand_seed() -> u64 {
        0
    }

    pub fn store(&self) {
    }

    pub fn initialize() {
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
        unsafe {STATE.store()};
        data
    }
}

pub static mut STATE: State  = State {};

pub struct Transaction {
    pub command: u64,
    pub data: Vec<u64>,
}

// 简化命令常量
const INSTALL_PLAYER: u64 = 1;
const GENERATE_RAND: u64 = 2;
const REVEAL_RAND: u64 = 3;  // 合并了原来的 SUBMIT_SIGNATURE 和 REVEAL_SEED

// 简化错误常量
const ERROR_PLAYER_ALREADY_EXIST: u32 = 1;
const ERROR_PLAYER_NOT_EXIST: u32 = 2;
const ERROR_NO_COMMITMENT: u32 = 3;
const ERROR_INVALID_SEED: u32 = 5;

impl Transaction {
    pub fn decode_error(e: u32) -> &'static str {
        match e {
           ERROR_PLAYER_NOT_EXIST => "PlayerNotExist",
           ERROR_PLAYER_ALREADY_EXIST => "PlayerAlreadyExist",
           _ => "Unknown"
        }
    }
    pub fn decode(params: [u64; 4]) -> Self {
        let command = (params[0] >> 32) & 0xff;
        let data = vec![params[1], params[2], params[3]]; // pkey[0], pkey[1], amount
        Transaction {
            command,
            data,
        }
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

    /*
    pub fn generate_rand(&self, pkey: &[u64; 4]) -> u32 {
        let pid = DolphinPlayer::pkey_to_pid(pkey);
        let mut player = match DolphinPlayer::get_from_pid(&pid) {
            Some(p) => p,
            None => return ERROR_PLAYER_NOT_EXIST,
        };

        // 生成新的 seed_info
        player.data.seed_info = Some(SeedInfo::generate_seed_commitment());
        player.data.final_random = None;
        player.store();
        0
    }

    pub fn reveal_rand(&self, pkey: &[u64; 4]) -> u32 {
        let pid = DolphinPlayer::pkey_to_pid(pkey);
        let mut player = match DolphinPlayer::get_from_pid(&pid) {
            Some(p) => p,
            None => return ERROR_PLAYER_NOT_EXIST,
        };

        let seed_info = match &player.data.seed_info {
            Some(info) => info,
            None => return ERROR_NO_COMMITMENT,
        };

        // 假设玩家签名在 self.data[0] 中
        let player_signature = self.data[0];

        // 验证并生成随机数
        match seed_info.reveal_verify_and_generate_random(player_signature) {
            Ok(random) => {
                player.data.final_random = Some(random);
                player.store();
                0
            },
            Err(_) => ERROR_INVALID_SEED,
        }
    }

    */
    pub fn process(&self, pkey: &[u64; 4], _rand: &[u64; 4]) -> u32 {
        let b = match self.command {
            INSTALL_PLAYER => self.install_player(pkey),
            /*
            GENERATE_RAND => self.generate_rand(pkey),
            REVEAL_RAND => self.reveal_rand(pkey),
            */
            _ => 0
        };
        let kvpair = unsafe { &mut MERKLE_MAP.merkle.root };
        zkwasm_rust_sdk::dbg!("root after process {:?}\n", kvpair);
        b
    }
}
