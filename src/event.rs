use std::{collections::LinkedList, slice::IterMut};
use zkwasm_rest_abi::{StorageData, MERKLE_MAP};

use crate::{
    gameplay::Dolphin, state::DolphinPlayer
};
use crate::gameplay::PlayerData;

#[derive(Clone)]
pub struct Event {
    pub pid: [u64; 2],
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
            object_index: (f >> 32) as usize,
            delta: (f & 0xffffffff) as usize,
        }
    }
}

pub struct EventQueue {
    pub counter: u64,
    pub list: std::collections::LinkedList<Event>,
}

pub fn apply_dolphin_event(player: &mut DolphinPlayer, object: &mut Dolphin) -> Result<(), u32> {
    todo!()
}

impl StorageData for EventQueue {
    fn to_data(&self, buf: &mut Vec<u64>) {
        buf.push(self.counter);
        for e in self.list.iter() {
            e.to_data(buf);
        }
        let kvpair = unsafe { &mut MERKLE_MAP };
    }
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
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
    pub fn tick(&mut self) {
        self.dump();
        let counter = self.counter;
        while let Some(head) = self.list.front_mut() {
            if head.delta == 0 {
                let objindex = head.object_index;
                let mut player = DolphinPlayer::get_from_pid(&head.pid).unwrap();
                let dolphin = player.data;
            } else {
                head.delta -= 1;
                break;
            }
        }
        self.counter += 1;
    }

    pub fn insert(
        &mut self,
        object_index: usize,
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
