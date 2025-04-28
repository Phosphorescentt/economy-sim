use crate::{
    actors::{Action, Actor},
    engine::ActionResponse,
    orders::CounterpartyCode,
};

pub struct NoopActor {
    counterparty_code: CounterpartyCode,
}

impl From<CounterpartyCode> for NoopActor {
    fn from(counterparty_code: CounterpartyCode) -> Self {
        Self { counterparty_code }
    }
}

impl Actor for NoopActor {
    fn counterparty_code(self) -> CounterpartyCode {
        self.counterparty_code
    }

    fn counterparty_code_as_ref(&self) -> &CounterpartyCode {
        &self.counterparty_code
    }

    fn act(&self) -> Action {
        Action::Noop
    }

    fn register_action_response(&mut self, _action_response: ActionResponse) -> () {
        ()
    }
}
