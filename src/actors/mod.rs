mod noop_actor;
mod simple_actor;

use crate::{
    engine::ActionResponse,
    exchanges::ExchangeCode,
    orders::{CounterpartyCode, Order, OrderId},
};
pub use noop_actor::NoopActor;
pub use simple_actor::SimpleActor;

#[derive(Debug)]
pub enum Action {
    Noop,
    SubmitOrder(ExchangeCode, Order),
    RetractOrder(ExchangeCode, OrderId),
}

pub trait Actor {
    fn counterparty_code(self) -> CounterpartyCode;
    fn counterparty_code_as_ref(&self) -> &CounterpartyCode;
    fn act(&self) -> Action;
    fn register_action_response(&mut self, action_response: ActionResponse) -> ();
}
