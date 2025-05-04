use std::collections::{HashMap, HashSet};

use crate::{
    exchanges::ExchangeOrder,
    order_collections::OrderCollection,
    trades::{Trade, TradeId},
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderId(pub u32);

impl OrderId {
    pub fn next(self) -> OrderId {
        OrderId(self.0 + 1)
    }
}

impl OrderId {
    fn new() -> OrderId {
        OrderId(0)
    }
}

pub struct CompositeOrderId {
    pub ticker: Ticker,
    pub order_id: OrderId,
}

impl From<&str> for Ticker {
    fn from(value: &str) -> Self {
        Ticker(String::from(value))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CounterpartyCode(pub String);

impl From<&str> for CounterpartyCode {
    fn from(value: &str) -> Self {
        CounterpartyCode(String::from(value))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Ticker(pub String);

#[derive(Clone, PartialEq, Debug)]
pub struct Price(pub f32);
impl Eq for Price {}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        f32::partial_cmp(&self.0, &other.0)
    }
}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        f32::total_cmp(&self.0, &other.0)
    }
}

#[derive(Clone, Debug)]
pub enum OrderDirection {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub price: Price,
    pub ticker: Ticker,
    pub counterparty_code: CounterpartyCode,
    pub direction: OrderDirection,
}

pub struct OrderBook {
    ticker: Ticker,
    all_orders: HashMap<OrderId, ExchangeOrder>,
    bid_prices: OrderCollection,
    ask_prices: OrderCollection,
    latest_order_id: OrderId,
    latest_trade_id: TradeId,
}

impl OrderBook {
    fn new_order_id(&mut self) -> OrderId {
        let new_order_id = self.latest_order_id.clone().next();
        self.latest_order_id = new_order_id.clone();
        new_order_id
    }

    fn new_trade_id(&mut self) -> TradeId {
        let new_trade_id = self.latest_trade_id.clone().next();
        self.latest_trade_id = new_trade_id.clone();
        new_trade_id
    }

    pub fn add_order(&mut self, order: Order) -> CompositeOrderId {
        let direction = order.direction.clone();
        let new_order_id = self.new_order_id();
        let exchange_order = ExchangeOrder {
            order_id: new_order_id.clone(),
            order,
        };

        // Register the order in our map of all orders.
        self.all_orders
            .insert(new_order_id.clone(), exchange_order.clone());

        // Register the prices with the respective prices.
        match direction {
            OrderDirection::Bid => self.bid_prices.insert(exchange_order),
            OrderDirection::Ask => self.ask_prices.insert(exchange_order),
        }
        CompositeOrderId {
            ticker: self.ticker.clone(),
            order_id: new_order_id,
        }
    }

    pub fn match_orders(&mut self) -> Vec<(TradeId, Trade)> {
        let mut id_hash_set: HashSet<OrderId> = HashSet::new();
        let mut bid_ask_ids: Vec<(OrderId, OrderId)> = Vec::new();
        for bid_price in self.bid_prices.item_set.iter() {
            if id_hash_set.contains(&bid_price.id) {
                // If we have already seeen the ID, then continue.
                continue;
            }

            // Iterate backwards so that we always find the optimal match.
            for ask_price in self.ask_prices.item_set.iter().rev() {
                // If we have already seeen the ID, then continue.
                if id_hash_set.contains(&ask_price.id) {
                    continue;
                }

                if bid_price.price == ask_price.price {
                    id_hash_set.insert(bid_price.id);
                    id_hash_set.insert(ask_price.id);

                    bid_ask_ids.push((bid_price.id, ask_price.id.clone()));
                    break;
                }
            }
        }

        let mut trades: Vec<(TradeId, Trade)> = Vec::new();
        for (bid_id, ask_id) in bid_ask_ids {
            let bid_order = self.all_orders.remove(&bid_id).unwrap();
            let ask_order = self.all_orders.remove(&ask_id).unwrap();
            assert_eq!(bid_order.order.price, ask_order.order.price);

            let bid_price = self.bid_prices.remove(bid_order.order_id).unwrap();
            let ask_price = self.ask_prices.remove(ask_order.order_id).unwrap();
            assert_eq!(bid_price.price, ask_price.price);

            trades.push((
                self.new_trade_id(),
                Trade {
                    buyer: bid_order.order.counterparty_code,
                    seller: ask_order.order.counterparty_code,
                    ticker: self.ticker.clone(),
                    price: bid_price.price,
                },
            ))
        }

        trades
    }
}

impl From<Ticker> for OrderBook {
    fn from(ticker: Ticker) -> Self {
        OrderBook {
            ticker: ticker,
            all_orders: HashMap::new(),
            bid_prices: OrderCollection::from(OrderDirection::Bid),
            ask_prices: OrderCollection::from(OrderDirection::Ask),
            latest_order_id: OrderId::new(),
            latest_trade_id: TradeId::new(),
        }
    }
}
