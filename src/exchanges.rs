use crate::orders::{Order, Price};
use std::collections::HashMap;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExchangeCode(pub String);

pub struct Exchange {
    name: String,
    pub code: ExchangeCode,
    orders: HashMap<Price, Vec<Order>>,
}

impl Exchange {
    pub fn from_exchange_code(code: ExchangeCode) -> Self {
        Self {
            name: code.0.clone(),
            code,
            orders: HashMap::new(),
        }
    }

    pub fn submit_order(&mut self, order: Order) -> () {
        if let Some(mut existing_orders) = self.orders.get_mut(&order.price) {
            existing_orders.push(order);
        } else {
            self.orders.insert(order.price.clone(), vec![order]);
        }
    }
}
