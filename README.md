# Market Simulator

## Todo
- [x] Make state only update after time step has finished to ensure that no weirdness happens due to execution order.
  - This was achieved by adding a pre-action step in which actors can query the market for data.
- [x] Implement SubmitOrder action
- [x] Implement exchange order matching.
  [Documentation from CME on order matching](https://cmegroupclientsite.atlassian.net/wiki/spaces/EPICSANDBOX/pages/457218479/Supported+Matching+Algorithms)
  This might warrant refactoring exchanges to be behind a trait so that we can have
different matching implementations.
- [ ] Engine should reject Actions that are invalid. I.e. an Actor attempting to submit
  an order for a trade with a higher notional than the Actor has.
  - [ ] Refactor `Exchange.bid_orders` and `Exchange.ask_orders` to be ordered hash
  maps.
- [ ] WASM bindings for Actors?
- [ ] WASM bindings for Exchanges?
- [ ] Serialise exchange/market history to a file
