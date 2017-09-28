use std::cmp::Ordering;
use trade::*;

#[derive(Copy, Clone, Debug)]
pub struct Order {
    pub entrust_id: u64,
    pub direction: Direction,
    pub price: Price,
    pub volume: Volume
}

/// ordered by price, larger means more likely to get traded
/// a wrapper for Order to work in `BinaryHeap`
#[derive(Copy, Clone)]
pub struct AskOrder(pub Order);

/// ordered by price, larger means more likely to get traded
/// a wrapper for Order to work in `BinaryHeap`
#[derive(Copy, Clone)]
pub struct BidOrder(pub Order);

impl PartialEq for Order {
    fn eq(&self, other: &Order) -> bool {
        self.entrust_id == other.entrust_id
    }
}
impl Eq for Order {}

impl PartialEq for AskOrder {
    fn eq(&self, other: &AskOrder) -> bool {
        self.0.price.eq(&other.0.price)
    }
}

impl Eq for AskOrder {}

impl PartialOrd for AskOrder {
    fn partial_cmp(&self, other: &AskOrder) -> Option<Ordering> {
        // reversed, smaller the price, more likely to get traded
        other.0.price.partial_cmp(&self.0.price)
    }
}

impl Ord for AskOrder {
    fn cmp(&self, other: &AskOrder) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for BidOrder {
    fn eq(&self, other: &BidOrder) -> bool {
        self.0.price.eq(&other.0.price)
    }
}

impl Eq for BidOrder {}

impl PartialOrd for BidOrder {
    fn partial_cmp(&self, other: &BidOrder) -> Option<Ordering> {
        self.0.price.partial_cmp(&other.0.price)
    }
}

impl Ord for BidOrder {
    fn cmp(&self, other: &BidOrder) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Order {
    pub fn can_trade(&self, other: &Order) -> bool {
        match (self.direction, other.direction) {
            (Direction::Sell, Direction::Buy) => self.price < other.price,
            (Direction::Buy, Direction::Sell) => self.price > other.price,
            (_, _) => false
        }
    }
}