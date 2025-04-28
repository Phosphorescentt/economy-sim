use crate::actors::{Action, Actor};
use crate::exchanges::{Exchange, ExchangeCode};
use crate::orders::CounterpartyCode;
use std::collections::HashMap;
use std::error::Error;

pub enum ActionResponse {
    Noop,
    SubmitOrder(Option<(ExchangeCode, u32)>),
}

pub struct Engine {
    exchanges: HashMap<ExchangeCode, Exchange>,
    actors: HashMap<CounterpartyCode, Box<dyn Actor>>,
    actors_funds: HashMap<CounterpartyCode, i32>,
    time_horizon: u32,
}

impl From<u32> for Engine {
    fn from(value: u32) -> Self {
        Self {
            exchanges: HashMap::new(),
            actors: HashMap::new(),
            actors_funds: HashMap::new(),
            time_horizon: value,
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            exchanges: HashMap::new(),
            actors: HashMap::new(),
            actors_funds: HashMap::new(),
            time_horizon: 1000,
        }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        for _time in 0..self.time_horizon {
            for actor in self.actors.values().into_iter() {
                // Might have to redo the engine struct so that
                // `Actors` is a `Vec<Box<dyn Actor>>` instead of a HashMap
                // so that I can
                let action_response = match actor.act() {
                    Action::Noop => ActionResponse::Noop,
                    Action::SubmitOrder(exchange_code, order) => {
                        // TODO: implement this:
                        // 1. find correct exchange
                        // 2. submit order on that exchange
                        ActionResponse::Noop
                    }
                };

                *actor.register_action_response(action_response);
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
