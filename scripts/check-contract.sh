#!/usr/bin/env bash
. "$(dirname -- "$0")/lib.sh"
exec "$CARGO" run --quiet --locked -p hermes-contract-gen -- --check
