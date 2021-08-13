# <h1 align="center"> dark-forest.rs </h1>

*Terminal UI implementation and types for the Dark Forest game*

![Github Actions](https://github.com/gakonst/dark-forest/workflows/Tests/badge.svg)

## Development

We use the standard Rust toolchain

```
cargo check
cargo test
cargo doc --open
cargo run
```

## Move CLI Usage

In order to send forces to another planet, you can use the CLI command below, which
will move 100 units (internally coded as 100000, because DF types are all x100 on-chain to avoid
rounding errors) to the target planet. Coordinates are given as a comma separated list of
signed integers.

```
cargo r move --from=-100,100 --to=-120,120 \
    --wasm ./crates/df-engine/round3-data/move.wasm \
    --r1cs ./crates/df-engine/round3-data/move.r1cs \
    --zkey ./crates/df-engine/round3-data/move.zkey \
    --private-key <your private key without a 0x>
```

## Roadmap

* [ ] Dark Forest Types
    * [x] Map: Can read the map from the plugin
    * [ ] Planets
        * [x] Info
        * [x] Off-chain stats calculation for uninitialized planets
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
