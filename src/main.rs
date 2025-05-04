mod actors;
mod engine;
mod exchanges;
mod order_collections;
mod orders;
mod trades;

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
            ExchangeCode::from("ABCD"),
            Order {
                counterparty_code: CounterpartyCode::from("ABCD"),
                direction: OrderDirection::Bid,
                ticker: Ticker::from("AAPL"),
                price: Price(1.0),
            },
        ))))
        .add_actor(Box::new(SimpleActor::from((
            ExchangeCode::from("ABCD"),
            Order {
                counterparty_code: CounterpartyCode::from("EFGH"),
                direction: OrderDirection::Ask,
                ticker: Ticker::from("AAPL"),
                price: Price(1.0),
            },
        ))));

    let _ = engine.run();
}
