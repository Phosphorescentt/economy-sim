use crate::{
    engine::ActionResponse,
    orders::{Order, OrderId, Price, SubmittedOrder},
};
use std::collections::HashMap;
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExchangeCode(pub String);

pub struct Exchange {
    name: String,
    pub code: ExchangeCode,
    // NOTE: Maybe we split this out into Bid orders and Ask orders?
    orders: HashMap<Price, Vec<SubmittedOrder>>,
    latest_order_id: OrderId,
}

impl Exchange {
    pub fn from_exchange_code(code: ExchangeCode) -> Self {
        Self {
            name: code.0.clone(),
            code,
            orders: HashMap::new(),
            latest_order_id: OrderId(0),
        }
    }

    pub fn submitted_order_from_order(&mut self, order: Order) -> SubmittedOrder {
        self.latest_order_id = self.latest_order_id.clone().next();
        SubmittedOrder {
            id: self.latest_order_id.clone(),
            order: order,
        }
    }

    pub fn submit_order(&mut self, order: Order) -> ActionResponse {
        let order_price = order.price.clone();
        let submitted_order = self.submitted_order_from_order(order);
        let new_order_id = submitted_order.id.clone();

        if let Some(mut existing_orders) = self.orders.get_mut(&order_price) {
            existing_orders.push(submitted_order);
        } else {
            self.orders.insert(order_price, vec![submitted_order]);
        }
        ActionResponse::OrderSubmitted(self.code.clone(), new_order_id)
    }

    pub fn match_orders(&mut self) -> () {
        // iterate through all orders and attempt to match
        // any that do match, oragnise to send a message out to the actors that
        // their order has been matched
        // Keep a log of all the matches that occur so they can be replayed later.
    }
}
