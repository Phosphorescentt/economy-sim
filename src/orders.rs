#[derive(Clone, Debug, Hash)]
pub struct OrderId(pub u32);

impl OrderId {
    pub fn new() -> OrderId {
        OrderId(0)
    }

    pub fn next(self) -> OrderId {
        OrderId(self.0 + 1)
    }
}

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

impl From<&str> for Ticker {
    fn from(value: &str) -> Self {
        Ticker(String::from(value))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CounterpartyCode(pub String);

impl From<&str> for CounterpartyCode {
    fn from(value: &str) -> Self {
        CounterpartyCode(String::from(value))
    }
}

#[derive(Clone, Debug)]
pub struct Ticker(pub String);

#[derive(Clone, PartialEq, Debug)]
pub struct Price(pub f32);

#[derive(Clone, Debug)]
pub enum OrderDirection {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct Order {
    pub counterparty_code: CounterpartyCode,
    pub ticker: Ticker,
    pub price: Price,
    pub direction: OrderDirection,
}
