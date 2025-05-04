use std::{
    array::IntoIter,
    collections::{btree_set::Iter, BTreeSet, HashMap},
};

use crate::{
    exchanges::ExchangeOrder,
    orders::{OrderDirection, OrderId, Price},
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PriceIdPair {
    pub price: Price,
    pub id: OrderId,
}

pub struct OrderCollection {
    // This struct is used purely for matching prices. When you need to look up extra data for a
    // trade such as the counterparty_code, this should be stored somewhere else.
    id_price_map: HashMap<OrderId, Price>,
    pub item_set: BTreeSet<PriceIdPair>,
    direction: OrderDirection,
}

impl OrderCollection {
    pub fn insert(&mut self, exchange_order: ExchangeOrder) {
        let id = exchange_order.order_id;
        let price = exchange_order.order.price;
        let price_id_pair = PriceIdPair {
            price: price.clone(),
            id: id.clone(),
        };

        self.id_price_map.insert(id, price.clone());
        self.item_set.insert(price_id_pair);
    }

    pub fn remove(&mut self, order_id: OrderId) -> Option<PriceIdPair> {
        if let Some(price) = self.id_price_map.remove(&order_id) {
            let pair = PriceIdPair {
                price,
                id: order_id,
            };
            if self.item_set.remove(&pair) {
                Some(pair)
            } else {
                // If we get here, then at some point we have managed to remove something from the
                // item_set without removing it from the id_price_map. This is bad!
                panic!("Whoopsie!")
            }
        } else {
            None
        }
    }
}

impl From<OrderDirection> for OrderCollection {
    fn from(value: OrderDirection) -> Self {
        OrderCollection {
            id_price_map: HashMap::new(),
            item_set: BTreeSet::new(),
            direction: value,
        }
    }
}
