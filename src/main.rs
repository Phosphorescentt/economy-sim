mod actors;
mod engine;
mod exchanges;
mod orders;

use actors::{NoopActor, SimpleActor};
use engine::Engine;
use exchanges::{Exchange, ExchangeCode};
use log::info;
use orders::{CounterpartyCode, Order, OrderDirection, Price, Ticker};

fn main() {
    env_logger::init();
    info!("Logging initialised.");

    let engine = Engine::from(10)
        .add_exchange(Exchange::from_exchange_code(ExchangeCode(String::from(
            "ABCD",
        ))))
        .add_actor(Box::new(NoopActor::from(CounterpartyCode(String::from(
            "PQRS",
        )))))
        .add_actor(Box::new(SimpleActor::from((
            ExchangeCode(String::from("ABCD")),
            Order {
                counterparty_code: CounterpartyCode(String::from("WXYZ")),
                ticker: Ticker(String::from("AAPL")),
                direction: OrderDirection::Bid,
                price: Price::from(1.0),
                size: 1,
            },
        ))));

    let _ = engine.run();
}
