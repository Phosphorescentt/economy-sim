use crate::orders::{CounterpartyCode, Price, Ticker};

#[derive(Clone, Debug, Hash)]
pub struct TradeId(pub u32);

impl TradeId {
    pub fn new() -> TradeId {
        TradeId(0)
    }

    pub fn next(self) -> TradeId {
        TradeId(self.0 + 1)
    }
}

pub struct Trade {
    pub buyer: CounterpartyCode,
    pub seller: CounterpartyCode,
    pub ticker: Ticker,
    pub price: Price,
}
