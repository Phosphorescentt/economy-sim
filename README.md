# Economy Simulator

## Todo
- [ ] Implement exchange order matching.
- [x] Make state only update after time step has finished to ensure that no weirdness happens due to execution order.
  - This was achieved by adding a pre-action step in which actors can query the market for data.
- [x] Implement SubmitOrder action
