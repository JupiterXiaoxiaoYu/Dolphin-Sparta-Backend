use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;
use zkwasm_rust_sdk::{require, wasm_dbg, wasm_input};

//基础玩法，没有实现抽卡(随机数）、阶梯化价格（modifier）、成长（时间序列）、收货金币（时间序列）
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Command {
    BuyDolphin = 0,
    BuyFood = 1,
    BuyMedicine = 2,
    FeedDolphin = 3,
    HealDolphin = 4,
    AttackEvilWhale = 5,
    BuyPopulationNumber = 6,
}

impl Command {
    fn from_u8(value: u8) -> Option<Command> {
        match value {
            0 => Some(Command::BuyDolphin),
            1 => Some(Command::BuyFood),
            2 => Some(Command::BuyMedicine),
            3 => Some(Command::FeedDolphin),
            4 => Some(Command::HealDolphin),
            5 => Some(Command::AttackEvilWhale),
            6 => Some(Command::BuyPopulationNumber),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum LifeStage {
    Baby = 0,
    Adult = 1
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum DolphinName {
    DolphinArcher = 0,
    DolphinKnight = 1,
    DolphinMage = 2,
    DolphinPriest = 3,
    DolphinRogue = 4,
    DolphinWarrior = 5,
    DolphinWizard = 6,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Dolphin {
    id: u64,
    name: DolphinName,
    level: u64,
    life_stage: LifeStage,
    join_time: u64,
    health: u64,
    satiety: u64,
    generated_coins: u64,
    collected_coins: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Player {
    pkey: [u64; 4],
    coins_balance: u64,
    food_number: u64,
    medicine_number: u64,
    population_number: u64,
    dolphins: Vec<Dolphin>,
}

// update state via command, Command：购买海豚，购买食物，购买药品，喂食海豚[I]，喂药海豚[I], 迎击邪恶巨鲸，购买栏位(population_number)
#[wasm_bindgen]
pub fn update_state(command: u8, player: JsValue, dolphin_id: u64) -> JsValue {
    let mut player: Player = player.into_serde().unwrap();
    match Command::from_u8(command) {
        Some(command) => match command {
            Command::BuyDolphin => {
                unsafe {
                    require(player.coins_balance >= 100);
                    require(!(player.dolphins.len() as u64 >= player.population_number));
                }
                player.coins_balance -= 100;
                player.dolphins.push(Dolphin {
                    id: player.dolphins.len() as u64,
                    name: DolphinName::DolphinArcher,
                    level: 1,
                    life_stage: LifeStage::Baby,
                    join_time: 0,
                    satiety: 100,
                    health: 100,
                    generated_coins: 0,
                    collected_coins: 0,
                });
            }
            Command::BuyFood => {
                unsafe {
                    require(player.coins_balance >= 10);
                }
                player.coins_balance -= 10;
                player.food_number += 1;
            }
            Command::BuyMedicine => {
                unsafe {
                    require(player.coins_balance >= 20);
                }
                player.coins_balance -= 20;
                player.medicine_number += 1;
            }
            Command::FeedDolphin => {
                unsafe {
                    require(player.food_number > 0);
                    require(player.dolphins.len() > 0);
                }
                player.food_number -= 1;
                player.dolphins[dolphin_id as usize].satiety += 10;
            }
            Command::HealDolphin => {
                unsafe {
                    require(player.medicine_number > 0);
                    require(player.dolphins.len() > 0);
                }
                player.medicine_number -= 1;
                player.dolphins[dolphin_id as usize].health = 100;
            }
            Command::AttackEvilWhale => {
                unsafe {
                    require(player.dolphins.len() > 0);
                }
                //all dolphins health to 0
                for dolphin in player.dolphins.iter_mut() {
                    dolphin.health = 0;
                }
                //collect all coins
                for dolphin in player.dolphins.iter_mut() {
                    player.coins_balance += dolphin.generated_coins;
                    dolphin.generated_coins = 0;
                    dolphin.collected_coins += dolphin.generated_coins;
                }
            }
            Command::BuyPopulationNumber => {
                unsafe {
                    require(player.coins_balance >= 100);
                }
                player.coins_balance -= 200;
                player.population_number += 1;
            }
            _ => {}
        },
        None => {}
    }
    JsValue::from_serde(&player).unwrap()
}

#[wasm_bindgen]
pub fn zkmain() {
    unsafe {
        let player = Player {
            pkey: [1, 2, 3, 4],
            coins_balance: 1000,
            food_number: 0,
            medicine_number: 0,
            population_number: 1,
            dolphins: vec![],
        };
        let command = wasm_input(1) as u8;
        let dolphin_id = wasm_input(2) as u64;
        let result = update_state(command, JsValue::from_serde(&player).unwrap(), dolphin_id);
        let result_player: Player = result.into_serde().unwrap();
        let expected_player = Player {
            pkey: [1, 2, 3, 4],
            coins_balance: 900,
            food_number: 0,
            medicine_number: 0,
            population_number: 1,
            dolphins: vec![Dolphin {
                id: 0,
                name: DolphinName::DolphinArcher,
                level: 1,
                life_stage: LifeStage::Baby,
                join_time: 0,
                satiety: 100,
                health: 100,
                generated_coins: 0,
                collected_coins: 0,
            }],
        };
        let c: bool = result_player == expected_player;
        wasm_dbg(c as u64);
    }
}
