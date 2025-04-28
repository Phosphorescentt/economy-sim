mod noop_actor;
mod simple_actor;

use crate::{
    engine::ActionResponse,
    exchanges::ExchangeCode,
    orders::{CounterpartyCode, Order},
};
pub use noop_actor::NoopActor;
pub use simple_actor::SimpleActor;

pub enum Action {
    Noop,
    SubmitOrder(ExchangeCode, Order),
}

pub trait Actor {
    fn counterparty_code(self) -> CounterpartyCode;
    fn counterparty_code_as_ref(&self) -> &CounterpartyCode;
    fn act(&self) -> Action;
    fn register_action_response(&mut self, action_response: ActionResponse) -> ();
}
