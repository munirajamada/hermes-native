#!/usr/bin/env bash
. "$(dirname -- "$0")/lib.sh"
if [ "${1:-}" = "--fix" ]; then
  "$CARGO" fmt --all
  exec "$CARGO" clippy --workspace --all-targets --all-features -- -D warnings
fi
"$CARGO" fmt --all --check
exec "$CARGO" clippy --workspace --all-targets --all-features -- -D warnings
