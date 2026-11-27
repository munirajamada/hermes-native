#!/usr/bin/env bash
. "$(dirname -- "$0")/lib.sh"
"$ROOT/scripts/lint.sh"
"$ROOT/scripts/check-docs.sh"
"$ROOT/scripts/check-contract.sh"
exec "$ROOT/scripts/test.sh"
