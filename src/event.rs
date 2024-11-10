use std::{collections::LinkedList, slice::IterMut};
use zkwasm_rest_abi::{StorageData, MERKLE_MAP};
use std::cell::RefCell;


use crate::autotick;
use crate::{
    gameplay::Dolphin, state::DolphinPlayer
};
use crate::gameplay::PlayerData;

pub enum EventType {
    Grow = 0,
    Starve = 1,
    GenerateCoin = 2,
}

#[derive(Clone)]
pub struct Event {
    pub pid: [u64; 2],
    pub event_type: usize,
    pub object_index: usize,  // dolphin index
    pub delta: usize,
}


impl StorageData for Event {
    fn to_data(&self, buf: &mut Vec<u64>) {
        buf.push(self.pid[0]);
        buf.push(self.pid[1]);
        let encoded = ((self.event_type as u64) << 48) | ((self.object_index as u64) << 32) | (self.delta as u64);
        buf.push(encoded);
    }
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        let pid = [
            *u64data.next().unwrap(),
            *u64data.next().unwrap(),
        ];

        let f = *u64data.next().unwrap();
        Event {
            pid,
            event_type: (f >> 48) as usize,
            object_index: ((f >> 32) & 0xffff) as usize,
            delta: (f & 0xffffffff) as usize,
        }
    }
}

pub struct EventQueue {
    pub counter: u64,
    pub list: std::collections::LinkedList<Event>,
}

pub struct EventResult {
    pub object_id: usize,
    pub event_type: usize,
    pub owner: [u64; 2],
    pub delta: usize,
}

pub fn apply_dolphin_event(player: &mut DolphinPlayer, dolphin_position: usize, event_type: usize) -> Result<EventResult, u32> {
    zkwasm_rust_sdk::dbg!("=-=-= dolphin_position =-=-= {:?}\n", {dolphin_position});   
    zkwasm_rust_sdk::dbg!("=-=-= dolphin_len =-=-= {:?}\n", {player.data.dolphins.len()});
    let object = player.data.dolphins.get_mut(dolphin_position).ok_or(1u32)?;
    if event_type == EventType::Grow as usize {
        zkwasm_rust_sdk::dbg!("=-=-= grow =-=-= {:?}\n", {object.life_stage});
        if object.life_stage < 65 {
            object.life_stage += 35;
            zkwasm_rust_sdk::dbg!("=-=-= grow after =-=-= {:?}\n", {object.life_stage});
            Ok(EventResult {
                object_id: object.id as usize,
                event_type: EventType::Grow as usize,
                owner: player.player_id,
                delta: 3,
            })
            
        } else {
            object.life_stage = 100;
            Ok(EventResult {
                object_id: object.id as usize,
                event_type: EventType::GenerateCoin as usize,
                owner: player.player_id,
                delta: 3,
            })
        }
        // zkwasm_rust_sdk::dbg!("=-=-= grow after =-=-= {:?}\n", {object.life_stage});
        // zkwasm_rust_sdk::dbg!("=-=-= player_id =-=-= {:?}\n", {player.player_id});  
        // zkwasm_rust_sdk::dbg!("=-=-= object_id =-=-= {:?}\n", {object.id});
        // QUEUE.0.borrow_mut().insert(object.id as usize, EventType::GenerateCoin as usize, &player.player_id, 3);
    } else if event_type == EventType::Starve as usize {
        if(object.satiety<=3){
            object.satiety = 0;
            if(object.health<5){
                object.health = 0;
            }else{
                object.health -= 5;
            }
        }else{
            object.satiety -= 3;
        }
        Ok(EventResult {
            object_id: object.id as usize,
            event_type: EventType::Starve as usize,
            owner: player.player_id,
            delta: 3,
        })
        // QUEUE.0.borrow_mut().insert(object.id as usize, EventType::Starve as usize, &&player.player_id, 5);
    } else if event_type == EventType::GenerateCoin as usize {
        if(object.health>0){
            //max collected coins = 1000
            if(object.generated_coins+object.level*10>1000*object.level){
                object.generated_coins += 0;
            }else {
                object.generated_coins += object.level * 10;
            }
        }else {
            object.generated_coins += 0;
        }
        Ok(EventResult {
            object_id: object.id as usize,
            event_type: EventType::GenerateCoin as usize,
            owner: player.player_id,
            delta: 3,
        })
        // QUEUE.0.borrow_mut().insert(object.id as usize, EventType::GenerateCoin as usize, &&player.player_id, 5);
    } else {
        Err(2u32)
    }
}

impl StorageData for EventQueue {
    fn to_data(&self, buf: &mut Vec<u64>) {
        buf.push(self.counter);
        buf.push(self.list.len() as u64);
        for e in self.list.iter() {
            e.to_data(buf);
        }
        let kvpair = unsafe { &mut MERKLE_MAP };
    }
    fn from_data(u64data: &mut IterMut<u64>) -> EventQueue {
        let counter = *u64data.next().unwrap();
        let len = *u64data.next().unwrap() as usize;
        let mut list = LinkedList::new();
        for _ in 0..len {
            list.push_back(Event::from_data(u64data));
        }
        EventQueue {
            counter,
            list,
        }
    }
}

impl EventQueue {
    pub fn store(&mut self) {
        let mut data = Vec::new();
        self.to_data(&mut data);
        let kvpair = unsafe { &mut MERKLE_MAP };
        kvpair.set(&[0,0,0,0], data.as_slice());
    }
    pub fn fetch(&mut self) {
        let kvpair = unsafe { &mut MERKLE_MAP };
        let mut data = kvpair.get(&[0,0,0,0]);
        // Print the data
        zkwasm_rust_sdk::dbg!("fetch event queue\n");
        if !data.is_empty() {
            *self = EventQueue::from_data(&mut data.iter_mut());
            //打印事件队列
            self.dump();
        }
    }
    pub fn new() -> Self {
        EventQueue {
            counter: 0,
            list: LinkedList::new(),
        }
    }
    pub fn dump(&self) {
        zkwasm_rust_sdk::dbg!("=-=-= dump queue =-=-=\n");
        for m in self.list.iter() {
            let delta = m.delta;
            zkwasm_rust_sdk::dbg!("[{:?}] - delta: {:?} - object_index: {:?} - event_type: {:?}\n", {m.pid}, {m.delta}, {m.object_index}, {m.event_type});
        }
        zkwasm_rust_sdk::dbg!("=-=-= end =-=-=\n");
    }
    pub fn tick(&mut self) -> u32 {
        // self.dump();
        let mut event_list = Vec::new();
        let counter = self.counter;
        while let Some(head) = self.list.front_mut() {
            let objindex = head.object_index;
            let mut player = DolphinPlayer::get_from_pid(&head.pid).unwrap();
            let dolphin_ids = player.data.dolphins.iter().map(|d| d.id).collect::<Vec<_>>();
            if head.delta == 0 {
                if dolphin_ids.contains(&(objindex as u64)) {

                    let event_type = head.event_type;
                    zkwasm_rust_sdk::dbg!("=-=-= tick =-=-= pid: {:?} - object_index: {:?} - event_type: {:?}\n", {head.pid}, {head.object_index}, {head.event_type});
                    
                    //zkwasm_rust_sdk::dbg!("=-=-= player =-=-= {:?}\n", {player.data.pid});
                    zkwasm_rust_sdk::dbg!("=-=-= dolphin_len =-=-= {:?}\n", {player.data.dolphins.len()});
                    zkwasm_rust_sdk::dbg!("Looking for dolphin with id: {:?}\n", objindex);
                    
                    zkwasm_rust_sdk::dbg!("Available dolphin IDs: {:?}\n", dolphin_ids);
                    let dolphin_position = player.data.dolphins
                        .iter()
                        .position(|d| d.id == objindex as u64)
                        .ok_or_else(|| {
                            zkwasm_rust_sdk::dbg!("Failed to find dolphin with id: {:?}\n", objindex);
                            1u32
                        })
                        .unwrap() as usize;
                    zkwasm_rust_sdk::dbg!("=-=-= dolphin_position =-=-= {:?}\n", {dolphin_position});
                    let result = apply_dolphin_event(&mut player, dolphin_position, event_type);
                    player.store();
                    self.list.pop_front();
                    if let Ok(r) = result {
                        event_list.push(r);
                    }else {
                        break;
                    }
                }else{
                    self.list.pop_front();
                    continue;
                }
                
            } else {
                head.delta -= 1;
                break;
            }
        }

        for event in event_list.iter() {
            self.insert(event.object_id, event.event_type, &event.owner, event.delta);
        }
        self.counter += 1;
        self.counter as u32
    }

    pub fn insert(
        &mut self,
        object_index: usize,
        event_type: usize,
        owner: &[u64; 2],
        delta: usize,
    ) {
        let mut delta = delta;
        let mut list = LinkedList::new();
        let mut tail = self.list.pop_front();
        while tail.is_some() && tail.as_ref().unwrap().delta <= delta {
            delta = delta - tail.as_ref().unwrap().delta;
            list.push_back(tail.unwrap());
            tail = self.list.pop_front();
        }
        let node = Event {
            object_index,
            event_type,
            pid: owner.clone(),
            delta,
        };
        list.push_back(node);
        match tail.as_mut() {
            Some(t) => {
                t.delta = t.delta - delta;
                list.push_back(t.clone());
            }
            None => (),
        };
        list.append(&mut self.list);
        self.list = list;
    }
}
pub struct SafeEventQueue(pub RefCell<EventQueue>);
unsafe impl Sync for SafeEventQueue {}

lazy_static::lazy_static! {
    pub static ref QUEUE: SafeEventQueue = SafeEventQueue (RefCell::new(EventQueue::new()));
}
