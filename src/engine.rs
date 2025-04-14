use crate::actors::{Action, Actor};
use crate::exchanges::{Exchange, ExchangeCode};
use crate::orders::CounterpartyCode;
use std::collections::HashMap;
use std::error::Error;

pub struct Engine {
    exchanges: HashMap<ExchangeCode, Exchange>,
    actors: HashMap<CounterpartyCode, Box<dyn Actor>>,
    time_horizon: i32,
}

impl From<i32> for Engine {
    fn from(value: i32) -> Self {
        Self {
            exchanges: HashMap::new(),
            actors: HashMap::new(),
            time_horizon: value,
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            exchanges: HashMap::new(),
            actors: HashMap::new(),
            time_horizon: 1000,
        }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        for time in 0..self.time_horizon {
            for actor in self.actors.values().into_iter() {
                let action = actor.act();
                // NOTE: Create some notion of an `ActionResponse` that the engine
                // passes back to the actor. I.e. when an order is submitted to an
                // exchange, pass back a trade ID to the actor so it can keep track
                // of open trades or submit actions to close trades.
                let action_response = match action {
                    Action::Noop => { /* do nothing */ }
                    Action::SubmitOrder(exchange_code, order) => {
                        // TODO: implement this:
                        // 1. find correct exchange
                        // 2. submit order on that exchange
                    }
                };
            }
        }
        Ok(())
    }

    pub fn add_exchange(mut self, exchange: Exchange) -> Self {
        self.exchanges.insert(exchange.code.clone(), exchange);
        self
    }

    pub fn add_actor(mut self, actor: Box<dyn Actor>) -> Self {
        self.actors
            .insert(actor.counterparty_code_as_ref().clone(), actor);
        self
    }
}
