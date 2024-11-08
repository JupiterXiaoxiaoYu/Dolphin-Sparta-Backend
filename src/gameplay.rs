use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::slice::IterMut;
use zkwasm_rest_abi::StorageData;
use zkwasm_rust_sdk::{require, wasm_dbg};

//基础玩法，没有实现抽卡(随机数）、阶梯化价格（modifier）、成长（时间序列）、收货金币（时间序列）
#[derive(Serialize, Deserialize, Debug, PartialEq, TryFromPrimitive, Copy, Clone)]
#[repr(u32)]
enum Command {
    BuyDolphin = 0,
    BuyFood = 1,
    BuyMedicine = 2,
    FeedDolphin = 3,
    HealDolphin = 4,
    AttackEvilWhale = 5,
    BuyPopulationNumber = 6,
    CollectCoins = 7,
    AddCoinsForTestOnly = 8,
    SellDolphin = 9,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TryFromPrimitive, Copy, Clone)]
#[repr(u32)]
enum LifeStage {
    Baby = 0,
    Adult = 1,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TryFromPrimitive, Copy, Clone)]
#[repr(u32)]
enum DolphinName {
    DolphinPikeman = 0,
    DolphinWarrior = 1,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dolphin {
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

impl Default for Dolphin {
    fn default() -> Self {
        Dolphin {
            id: 0,
            name: DolphinName::DolphinWarrior,
            level: 1,
            life_stage: LifeStage::Baby,
            join_time: 0,
            health: 100,
            satiety: 70,
            generated_coins: 0,
            collected_coins: 0,
        }
    }
}

impl StorageData for Dolphin {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        Dolphin {
            id: *u64data.next().unwrap(),
            name: DolphinName::try_from(*u64data.next().unwrap() as u32).unwrap(),
            level: *u64data.next().unwrap(),
            life_stage: LifeStage::try_from(*u64data.next().unwrap() as u32).unwrap(),
            join_time: *u64data.next().unwrap(),
            health: *u64data.next().unwrap(),
            satiety: *u64data.next().unwrap(),
            generated_coins: *u64data.next().unwrap(),
            collected_coins: *u64data.next().unwrap(),
        }
    }

    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.id);
        data.push(self.name as u64);
        data.push(self.level);
        data.push(self.life_stage as u64);
        data.push(self.join_time);
        data.push(self.health);
        data.push(self.satiety);
        data.push(self.generated_coins);
        data.push(self.collected_coins);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PlayerData {
    pid: [u64; 2],
    coins_balance: u64,
    dolphin_token_balance: u64,
    dolphin_index: u64,
    food_number: u64,
    medicine_number: u64,
    population_number: u64,
    dolphins: Vec<Dolphin>,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            pid: [0, 0],
            dolphin_token_balance: 0,
            dolphin_index: 0,
            coins_balance: 2000,    // 修改默认值
            food_number: 10,        // 修改默认值
            medicine_number: 2,     // 修改默认值
            population_number: 3,   // 修改默认值
            dolphins: Vec::new(),
        }
    }
}

impl StorageData for PlayerData {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        let mut player_data = PlayerData {
            pid: [*u64data.next().unwrap(), *u64data.next().unwrap()],
            coins_balance: *u64data.next().unwrap(),
            dolphin_token_balance: *u64data.next().unwrap(),
            dolphin_index: *u64data.next().unwrap(),
            food_number: *u64data.next().unwrap(),
            medicine_number: *u64data.next().unwrap(),
            population_number: *u64data.next().unwrap(),
            dolphins: vec![],
        };
        let dsize = *u64data.next().unwrap() as usize;
        for _ in 0..dsize {
            player_data.dolphins.push(Dolphin::from_data(u64data));
        }
        player_data
    }
    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.pid[0]);
        data.push(self.pid[1]);
        data.push(self.coins_balance);
        data.push(self.dolphin_token_balance);
        data.push(self.dolphin_index);
        data.push(self.food_number);
        data.push(self.medicine_number);
        data.push(self.population_number);
        data.push(self.dolphins.len() as u64);
        for i in 0..self.dolphins.len() {
            self.dolphins[i].to_data(data);
        }
    }
}

// update state via command, Command：购买海豚，购买食物，购买药品，喂食海豚[I]，喂药海豚[I], 迎击邪恶巨鲸，购买栏位(population_number)
pub fn update_state(command: u64, player: &mut PlayerData, dolphin_id: u64, rand: u64) -> Result<(), u32> {
    Command::try_from(command as u32).map_or(Err(1), |command| {
        match command {
            Command::BuyDolphin => {
                let dolphin_name = match dolphin_id {
                    0 => DolphinName::DolphinPikeman,
                    _ => DolphinName::DolphinWarrior,
                };
                unsafe {
                    if dolphin_name == DolphinName::DolphinPikeman {
                        require(player.coins_balance >= 150);
                        player.coins_balance -= 150;
                    }else {
                        require(player.coins_balance >= 100);
                        player.coins_balance -= 100;
                    }
                    require(!(player.dolphins.len() as u64 >= player.population_number));
                }
                player.dolphins.push(Dolphin {
                    id: player.dolphin_index,
                    name: dolphin_name,
                    level: rand % 4 + 1,
                    life_stage: LifeStage::Baby,
                    join_time: 0,
                    satiety: 70,
                    health: 100,
                    generated_coins: 0,
                    collected_coins: 0,
                });
                player.dolphin_index += 1;
            }
            Command::BuyFood => {
                unsafe {
                    require(player.coins_balance >= 50);
                }
                player.coins_balance -= 50;
                player.food_number += 5;
            }
            Command::BuyMedicine => {
                unsafe {
                    require(player.coins_balance >= 150);
                }
                player.coins_balance -= 150;
                player.medicine_number += 5;
            }
            Command::FeedDolphin => {
                zkwasm_rust_sdk::dbg!("feed {:?}, dolphin id {:?}", command, dolphin_id);
                unsafe {
                    require(player.food_number > 0);
                    require(player.dolphins.len() > 0);
                    require(dolphin_id < player.dolphin_index);
                }

                // 找到对应 dolphin_id 的海豚在数组中的位置
                let dolphin_position = player.dolphins.iter()
                    .position(|d| d.id == dolphin_id)
                    .ok_or(1u32)?;
                
                player.food_number -= 1;
                if player.dolphins[dolphin_position].satiety <= 90 {
                    player.dolphins[dolphin_position].satiety += 10;
                } else {
                    player.dolphins[dolphin_position].satiety = 100;
                }
            }
            Command::HealDolphin => {
                zkwasm_rust_sdk::dbg!("heal {:?}, dolphin id {:?}", command, dolphin_id);
                unsafe {
                    require(player.medicine_number > 0);
                    require(player.dolphins.len() > 0);
                    require(dolphin_id < player.dolphin_index);
                }

                // 找到对应 dolphin_id 的海豚在数组中的位置
                let dolphin_position = player.dolphins.iter()
                    .position(|d| d.id == dolphin_id)
                    .ok_or(1u32)?;
                
                player.medicine_number -= 1;
                player.dolphins[dolphin_position].health = 100;
            }
            Command::AttackEvilWhale => {
                unsafe {
                    require(player.dolphins.len() > 0);
                }
                //all dolphins health to
                //collect all coins
                unsafe {
                    require(player.coins_balance >= 1000);
                }
                player.coins_balance -= 1000;

                for dolphin in player.dolphins.iter_mut() {
                    dolphin.health = 0;
                    dolphin.satiety = 0;    
                }
                player.dolphin_token_balance += 50;
            }
            Command::BuyPopulationNumber => {
                unsafe {
                    require(player.coins_balance >= 200);
                }
                player.coins_balance -= 200;
                player.population_number += 1;
            }
            Command::CollectCoins => {
                unsafe {
                    require(player.dolphins.len() > 0);
                }
                for dolphin in player.dolphins.iter_mut() {
                    player.coins_balance += dolphin.generated_coins;
                    dolphin.generated_coins = 0;
                    dolphin.collected_coins += dolphin.generated_coins;
                }
            }
            Command::AddCoinsForTestOnly => {
                player.coins_balance += 100;
            }
            Command::SellDolphin => {
                unsafe {
                    require(player.dolphins.len() > 0);
                    require(dolphin_id < player.dolphin_index);
                }
                
                // 找到对应 dolphin_id 的海豚在数组中的位置
                let dolphin_position = player.dolphins.iter()
                    .position(|d| d.id == dolphin_id)
                    .ok_or(1u32)?;  // 如果找不到返回错误
                zkwasm_rust_sdk::dbg!("dolphin_position{:?}", dolphin_position);
                let dolphin = &player.dolphins[dolphin_position];
                let sell_price = match dolphin.name {
                    DolphinName::DolphinPikeman => 75,  // 150的一半
                    DolphinName::DolphinWarrior => 50,  // 100的一半
                };
                
                player.coins_balance += sell_price;
                player.dolphins.remove(dolphin_position);
            }
        };
        zkwasm_rust_sdk::dbg!("player state update {:?}, dolphin id {:?}", command, dolphin_id);
        Ok(())
    })
}
