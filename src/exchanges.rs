use crate::{
    engine::ActionResponse,
    orders::{Order, OrderDirection, OrderId, TradeId},
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExchangeCode(pub String);

impl From<&str> for ExchangeCode {
    fn from(value: &str) -> Self {
        ExchangeCode(String::from(value))
    }
}

pub struct Exchange {
    name: String,
    pub code: ExchangeCode,
    /// A list of bid orders, sorted with descending price.
    bid_orders: Vec<ExchangeOrder>,
    /// A list of ask orders, sorted with ascending price.
    ask_orders: Vec<ExchangeOrder>,
    latest_order_id: OrderId,
    latest_trade_id: TradeId,
}

pub struct ExchangeOrder {
    order_id: OrderId,
    order: Order,
}

impl Exchange {
    pub fn from_exchange_code(code: ExchangeCode) -> Self {
        Self {
            name: code.0.clone(),
            code,
            bid_orders: Vec::new(),
            ask_orders: Vec::new(),
            latest_order_id: OrderId::new(),
            latest_trade_id: TradeId::new(),
        }
    }

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

    fn add_bid_order(&mut self, bid_order: Order) -> ActionResponse {
        // This is probably extremely slow but I cba to write it properly right now :)
        let i = self
            .bid_orders
            .iter()
            .take_while(|existing_order| existing_order.order.price.0 > bid_order.price.0)
            .count();

        let order_id = self.new_order_id();
        self.bid_orders.insert(
            i,
            ExchangeOrder {
                order_id: order_id.clone(),
                order: bid_order,
            },
        );

        ActionResponse::OrderSubmitted(self.code.clone(), order_id)
    }

    fn add_ask_order(&mut self, ask_order: Order) -> ActionResponse {
        // This is probably extremely slow but I cba to write it properly right now :)
        let i = self
            .ask_orders
            .iter()
            .take_while(|existing_order| existing_order.order.price.0 > ask_order.price.0)
            .count();

        let order_id = self.new_order_id();
        self.ask_orders.insert(
            i,
            ExchangeOrder {
                order_id: order_id.clone(),
                order: ask_order,
            },
        );

        ActionResponse::OrderSubmitted(self.code.clone(), order_id)
    }

    pub fn submit_order(&mut self, order: Order) -> ActionResponse {
        match order.direction {
            OrderDirection::Bid => self.add_bid_order(order),
            OrderDirection::Ask => self.add_ask_order(order),
        }
    }
}
