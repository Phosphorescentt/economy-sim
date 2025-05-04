use crate::actors::{Action, Actor};
use crate::exchanges::{Exchange, ExchangeCode, ExchangeCompositeOrderId};
use crate::orders::CounterpartyCode;
use log::info;
use std::collections::HashMap;
use std::error::Error;

pub enum ActionResponse {
    Noop,
    OrderSubmitted(ExchangeCompositeOrderId),
    ExchangeCodeNotFound,
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

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        for time in 0..self.time_horizon {
            // Actor Update Step
            // This is when the engine updates the actors with events from the Exchanges.
            // I.e. matched orders.

            // Actor Action Step
            info!("Time step {}", time.to_string());
            for actor in self.actors.values_mut().into_iter() {
                let action = actor.act();
                info!(
                    "Actor '{:?}' is performing action '{:?}'",
                    actor.counterparty_code_as_ref(),
                    action
                );

                let action_response = match action {
                    Action::Noop => ActionResponse::Noop,
                    Action::SubmitOrder(exchange_code, order) => {
                        if let Some(exchange) = self.exchanges.get_mut(&exchange_code) {
                            exchange.submit_order(order)
                        } else {
                            ActionResponse::ExchangeCodeNotFound
                        }
                    }
                    Action::RetractOrder(exchange_code, order_id) => {
                        // This one is going to be a PITA because the way the data
                        // is stored makes it annoying to find an Order by ID.
                        todo!()
                    }
                    _ => todo!(),
                };

                actor.register_action_response(action_response);
            }

            // Exchange Matching Step
            // At this step we ask each of the exchanges to match orders.
            // Matched orders get added to the history and relevant counterparties
            // get notified that an order has been matched.
            for (code, exchange) in self.exchanges.iter_mut() {
                let matches = exchange.match_orders();
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
