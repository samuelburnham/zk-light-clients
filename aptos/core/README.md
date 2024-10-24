# Aptos Light ClientCore Crate

The `aptos-lc-core` crate contains most of the Aptos blockchain related data structures and logic that are used in the
light
client. The structures are heavily inspired from the [`aptos-core` codebase](https://github.com/aptos-labs/aptos-core).

## Codebase Structure

The codebase is split into several modules, each containing related data structures and logic:

- `aptos_test_utils`: This module contains test utilities for Aptos. It is only included when the `aptos` feature is
  enabled.
- `crypto`: This module contains cryptographic utilities used by the light client.
- `merkle`: This module contains data structures and utilities for working with Merkle trees.
- `types`: This module contains various data types used by the light client.

## Testing

There is a feature called `aptos` that can be leveraged to create tests against a mock implementation of the Aptos
chain. The code for this is located in the `aptos_test_utils` module.

To run tests, we recommend the following command:

```shell
SHARD_BATCH_SIZE=0 cargo nextest run --verbose --release --profile ci --features aptos --package aptos-lc --no-capture
```

This command should be run with the following environment variable:

- `RUSTFLAGS="-C target-cpu=native --cfg tokio_unstable -C opt-level=3"`:
    - `-C target-cpu=native`: This will ensure that the binary is optimized
      for the CPU it is running on. This is very important
      for [plonky3](https://github.com/plonky3/plonky3?tab=readme-ov-file#cpu-features) performance.
    - `--cfg tokio_unstable`: This will enable the unstable features of the
      Tokio runtime. This is necessary for aptos dependencies.
    - `-C opt-level=3`: This turns on the maximum level of compiler optimizations.
    - This can also be configured in `~/.cargo/config.toml` instead by adding:
        ```toml
        [target.'cfg(all())']
        rustflags = ["--cfg", "tokio_unstable", "-C", "target-cpu=native", "-C", "opt-level=3"]
        ```
- `SHARD_BATCH_SIZE=0`: Disables some checkpointing for faster proving at the cost of RAM.
