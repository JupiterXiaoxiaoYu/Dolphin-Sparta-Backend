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
        buf.push((self.object_index as u64) << 32 | (self.delta as u64));
    }
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        let pid = [
            *u64data.next().unwrap(),
            *u64data.next().unwrap(),
        ];

        let f = *u64data.next().unwrap();
        Event {
            pid,
            event_type: (f >> 32) as usize,
            object_index: (f >> 32) as usize,
            delta: (f & 0xffffffff) as usize,
        }
    }
}

pub struct EventQueue {
    pub counter: u64,
    pub list: std::collections::LinkedList<Event>,
}

pub fn apply_dolphin_event(player: &mut DolphinPlayer, dolphin_position: usize, event_type: usize) -> Result<(), u32> {
    let object = &mut player.data.dolphins[dolphin_position];
    if event_type == EventType::Grow as usize {
        if object.life_stage <= 80 {
            object.life_stage += 20;
        } else {
            object.life_stage = 100;
        }
        QUEUE.0.borrow_mut().insert(object.id as usize, EventType::GenerateCoin as usize, &player.data.pid, 5);
    } else if event_type == EventType::Starve as usize {
        object.satiety -= 10;
        if object.satiety < 0 {
            object.satiety = 0;
        }
        QUEUE.0.borrow_mut().insert(object.id as usize, EventType::Starve as usize, &player.data.pid, 5);
    } else if event_type == EventType::GenerateCoin as usize {
        object.generated_coins += object.level;
        QUEUE.0.borrow_mut().insert(object.id as usize, EventType::GenerateCoin as usize, &player.data.pid, 5);
    }
    Ok(())
}

impl StorageData for EventQueue {
    fn to_data(&self, buf: &mut Vec<u64>) {
        buf.push(self.counter);
        for e in self.list.iter() {
            e.to_data(buf);
        }
        let kvpair = unsafe { &mut MERKLE_MAP };
    }
    fn from_data(u64data: &mut IterMut<u64>) -> EventQueue {
        let counter = *u64data.next().unwrap();
        let mut list = LinkedList::new();
        for _ in 0..counter {
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
        if !data.is_empty() {
            EventQueue::from_data(&mut data.iter_mut());
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
            zkwasm_rust_sdk::dbg!("[{:?}] - {:?} - {}\n", {m.pid}, {m.delta}, {m.object_index});
        }
        zkwasm_rust_sdk::dbg!("=-=-= end =-=-=\n");
    }
    pub fn tick(&mut self) -> u32 {
        self.dump();
        let counter = self.counter;
        while let Some(head) = self.list.front_mut() {
            if head.delta == 0 {
                let objindex = head.object_index;
                let event_type = head.event_type;
                let mut player = DolphinPlayer::get_from_pid(&head.pid).unwrap();
                let dolphin_position = player.data.dolphins.iter().position(|d| d.id == objindex as u64).ok_or(1u32).unwrap() as usize;
                apply_dolphin_event(&mut player, dolphin_position, event_type);
            } else {
                head.delta -= 1;
                break;
            }
        }
        self.counter += 1;
        self.counter.try_into().unwrap()
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
