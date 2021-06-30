# <h1 align="center"> dark-forest.rs </h1>

*Terminal UI implementation and types for the Dark Forest game*

## Development

We use the standard Rust toolchain

```
cargo check
cargo test
cargo doc --open
cargo run
```

## Roadmap

* [ ] Dark Forest Types
    * [x] Map: Can read the map from the plugin
    * [ ] Planets
        * [x] Planet Info
* [ ] Actions
    * [ ] Planets
        * [ ] Moving
        * [ ] Upgrading
        * [ ] Refresh a Planet
    * [ ] Artifacts
        * [ ] Prospect
        * [ ] Find
        * [ ] Deposit
        * [ ] Withdraw
        * [ ] Activate
        * [ ] Deactive
    * [ ] Silver
* [ ] TUI / CLI / Repl for interacting with the system
* [ ] Caching responses
* [ ] Smart Contract Accounts
* [x] Type safe bindings to the smart contracts
* [ ] Fast Explorer
* [ ] Fast SNARKer for moves
* [ ] Refactor to smaller packages which can be imported for 3rd party integrations
