use crate::{
    actors::{Action, Actor},
    engine::ActionResponse,
    exchanges::ExchangeCode,
    orders::{CounterpartyCode, Order, OrderDirection, Price, Ticker},
};

pub struct SimpleActor {
    pub exchange_code: ExchangeCode,
    pub counterparty_code: CounterpartyCode,
    pub ticker: Ticker,
    pub direction: OrderDirection,
    pub price: Price,
    pub size: i32,
}

impl From<(ExchangeCode, Order)> for SimpleActor {
    fn from(exchange_code_order: (ExchangeCode, Order)) -> Self {
        Self {
            exchange_code: exchange_code_order.0,
            counterparty_code: exchange_code_order.1.counterparty_code,
            ticker: exchange_code_order.1.ticker,
            direction: exchange_code_order.1.direction,
            price: exchange_code_order.1.price,
            size: exchange_code_order.1.size,
        }
    }
}

impl Actor for SimpleActor {
    fn counterparty_code(self) -> CounterpartyCode {
        return self.counterparty_code;
    }

    fn counterparty_code_as_ref(&self) -> &CounterpartyCode {
        &self.counterparty_code
    }

    fn act(&self) -> Action {
        // Every time the simple actor acts, it just submits a new order.
        Action::SubmitOrder(
            self.exchange_code.clone(),
            Order {
                counterparty_code: self.counterparty_code.clone(),
                ticker: self.ticker.clone(),
                direction: self.direction.clone(),
                price: self.price.clone(),
                size: self.size,
            },
        )
    }

    fn register_action_response(&mut self, action_response: ActionResponse) -> () {
        // dump data into the VOID.
    }
}
