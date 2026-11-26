#!/usr/bin/env bash
. "$(dirname -- "$0")/lib.sh"
exec "$CARGO" build --workspace --locked "$@"
