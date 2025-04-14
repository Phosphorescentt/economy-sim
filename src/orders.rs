#[derive(Clone)]
pub enum OrderDirection {
    Bid,
    Ask,
}

fn integer_decode(val: f64) -> (u64, i16, i8) {
    // yikes.
    let bits: u64 = unsafe { std::mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CounterpartyCode(pub String);
#[derive(Clone)]
pub struct Ticker(pub String);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Price((u64, i16, i8));

impl From<f32> for Price {
    fn from(value: f32) -> Self {
        Self::from(value as f64)
    }
}

impl From<f64> for Price {
    fn from(value: f64) -> Self {
        Self(integer_decode(value))
    }
}

pub struct Order {
    pub counterparty_code: CounterpartyCode,
    pub ticker: Ticker,
    pub direction: OrderDirection,
    pub price: Price,
    pub size: i32,
}
