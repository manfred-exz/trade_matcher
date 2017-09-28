use std::collections::{HashMap, BinaryHeap};
use std::sync::Mutex;
use std::borrow::*;
use std::ops::Deref;
use std::sync::Arc;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Utc};
use trade::*;
use super::order::*;

pub enum MatchMode {
    AtOnceAnyPrice,
    AskBid,
}

pub struct MatchCenter {
    data: HashMap<SecurityUuid, SharedMatchQueue>,
    entrust_id_counter: EntrustIdCounter
}

pub struct SharedMatchQueue(Arc<Mutex<RefCell<MatchQueue>>>);

pub struct MatchQueue {
    ask: BinaryHeap<AskOrder>,
    bid: BinaryHeap<BidOrder>,
    entrust_id_counter: EntrustIdCounter
}

#[derive(Clone)]
pub struct EntrustIdCounter(Arc<Mutex<u64>>);

#[derive(Debug)]
pub struct MatchResult {
    pub ask: Order,
    pub bid: Order,
    pub trade_price: Price,
    pub trade_time: DateTime<Utc>,
}

impl MatchCenter {
    fn new(securities: &[SecurityUuid]) -> MatchCenter {
        let entrust_id_counter = EntrustIdCounter::new();
        let mut data: HashMap<SecurityUuid, SharedMatchQueue> =  HashMap::new();
        for security in securities {
            data.insert(security.clone(), SharedMatchQueue::new(entrust_id_counter.clone()));
        }

        MatchCenter {
            data,
            entrust_id_counter
        }
    }

    fn add_order(&self, security: &SecurityUuid, order: &Order) -> bool {
        if let Some(ref match_queue) = self.data.get(security) {
            match_queue.borrow_mut().add_order(order);
            true
        } else {
            false
        }
    }

    fn start_matching(&self, match_mode: &MatchMode) {
        loop {
            for match_queue in self.data.values() {
                if let Some(ref match_result) = match_queue.deref().borrow_mut().match_order(match_mode) {
                    println!("{:?}", match_result);
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}

impl SharedMatchQueue {
    pub fn new(entrust_id_counter: EntrustIdCounter) -> SharedMatchQueue {
        SharedMatchQueue(Arc::new(Mutex::new(RefCell::new(
            MatchQueue {
                ask: BinaryHeap::new(),
                bid: BinaryHeap::new(),
                entrust_id_counter
            }
        ))))
    }
}

impl Deref for SharedMatchQueue {
    type Target = RefCell<MatchQueue>;

    fn deref(&self) -> &Self::Target {
        &self.0.lock().unwrap()
    }
}

impl MatchQueue {
    pub fn new(entrust_id_counter: EntrustIdCounter) -> MatchQueue {
        MatchQueue{
            ask: BinaryHeap::new(),
            bid: BinaryHeap::new(),
            entrust_id_counter
        }
    }

    /// add order into match-queue, if order is matched, return the matched price
    pub fn add_order(&mut self, order: &Order) {
        match order.direction {
            Direction::Sell => self.ask.push(AskOrder(*order)),
            Direction::Buy => self.bid.push(BidOrder(*order))
        }
    }

    /// try to match the ask and bid order
    pub fn match_order(&mut self, mode: &MatchMode) -> Option<MatchResult> {
        match *mode {
            MatchMode::AskBid => self.match_ask_bid(),
            MatchMode::AtOnceAnyPrice => self.match_at_once_any_price(),
        }
    }

    fn match_at_once_any_price(&mut self) -> Option<MatchResult> {
        if let Some(ask_order) = self.ask.pop() {
            let fake_bid_order = Order{
                entrust_id: self.entrust_id_counter.take_and_inc_id(),
                direction: Direction::Buy,
                price: ask_order.0.price,
                volume: ask_order.0.volume,
            };
            Some(MatchResult::new(ask_order.0, fake_bid_order))
        } else {
            None
        }
    }

    fn match_ask_bid(&mut self) -> Option<MatchResult> {
        if let (Some(ask_order), Some(bid_order)) = (self.ask.peek().cloned(), self.bid.peek().cloned()) {
            if ask_order.0.can_trade(&bid_order.0) {
                // pop the traded order
                self.ask.pop();
                self.bid.pop();
                Some(MatchResult::new(ask_order.0, bid_order.0))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl MatchResult {
    fn new(ask: Order, bid: Order) -> MatchResult {
        let trade_price =
            if ask.entrust_id < bid.entrust_id {
                ask.price
            } else if bid.entrust_id < ask.entrust_id {
                bid.price
            } else {
                panic!("entrust id cannot be the same")
            };

        MatchResult {
            ask,
            bid,
            trade_price,
            trade_time: Utc::now()
        }
    }
}

impl EntrustIdCounter {
    pub fn new() -> EntrustIdCounter {
        EntrustIdCounter(Arc::new(Mutex::new(0)))
    }

    pub fn take_and_inc_id(&self) -> u64 {
        let mut x = self.0.lock().unwrap();
        {let old = *x; *x += 1; old}
    }
}