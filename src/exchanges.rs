use log::info;
use std::collections::HashMap;

use crate::{
    engine::ActionResponse,
    orders::{Order, OrderBook, OrderId, Ticker},
    trades::{Trade, TradeId},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExchangeCode(pub String);

impl From<&str> for ExchangeCode {
    fn from(value: &str) -> Self {
        ExchangeCode(String::from(value))
    }
}

// TODO: I think ideally bid_order and ask_order should be HashMap<OrderId, Order> with some special
// implementation of .iter() that iterates over the values with sorted prices. This makes the
// ergonomics much better as we can iterate in order and remove stuff in O(1) time.
pub struct Exchange {
    name: String,
    pub code: ExchangeCode,
    /// One order book per ticker.
    order_books: HashMap<Ticker, OrderBook>,
}

#[derive(Clone)]
pub struct ExchangeOrder {
    pub order_id: OrderId,
    pub order: Order,
}

pub struct ExchangeCompositeOrderId {
    exchange_code: ExchangeCode,
    ticker: Ticker,
    order_id: OrderId,
}

impl Exchange {
    pub fn from_exchange_code(code: ExchangeCode) -> Self {
        Self {
            name: code.0.clone(),
            code,
            order_books: HashMap::new(),
        }
    }

    pub fn submit_order(&mut self, order: Order) -> ActionResponse {
        info!("Exchange {:?} has recieved an order {:?}", self.code, order);
        // Find the book
        let mut book = self.order_books.get_mut(&order.ticker.clone());
        if book.is_none() {
            self.order_books
                .insert(order.ticker.clone(), OrderBook::from(order.ticker.clone()));
            book = self.order_books.get_mut(&order.ticker.clone());
        }

        // If we found a book, add the order!
        if let Some(book) = book {
            let composite_id = book.add_order(order);
            ActionResponse::OrderSubmitted(ExchangeCompositeOrderId {
                exchange_code: self.code.clone(),
                ticker: composite_id.ticker.clone(),
                order_id: composite_id.order_id.clone(),
            })
        } else {
            panic!("whoops!")
        }
    }

    pub fn match_orders(&mut self) -> Vec<(TradeId, Trade)> {
        // Very simple matching algo
        for (ticker, book) in self.order_books.iter_mut() {
            let trades = book.match_orders();
            info!(
                "Exchange {:?} matched {:?} trades with ticker {:?}",
                self.code,
                trades.len(),
                ticker
            );
        }
        Vec::new()
    }
}
