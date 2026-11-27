# Generate contract types from the vendored OpenRPC

`hermes-contract` is generated from `contract/gateway-contract.openrpc.json`, a vendored copy pinned to upstream `46c47112`. The output is committed and CI regenerates and diffs it. Hand-writing 265 method types would drift, and running the upstream Python generator would put Python in the Rust build.
