# rust-alttp-randomizer

The current lttp randomizer (http://vt.alttp.run/) codebase
is kinda clunky when it comes to small keys; lets fix that!

Also, let's make the set of randomizer-generated seeds exactly equal to the set
of guaranteed-beatable seeds.

This code uses the same basic item placement algorithm as the original code.
It's modified to explore all possible uses of small keys when deciding what
locations are available for placing the next item.

## how to run the code

There's no real output yet; my dev loop is to make changes and then pour through
the INFO/DEBUG log levels and make sure my generation algorithm isn't doing
anything obviously stupid:

```bash
cargo build --release
RUST_LOG=info NSIM=5 ./target/release/rando-rust
```

`NSIM` is number of seeds to create
