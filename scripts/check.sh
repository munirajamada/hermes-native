#!/usr/bin/env bash
. "$(dirname -- "$0")/lib.sh"
"$ROOT/scripts/lint.sh"
exec "$ROOT/scripts/test.sh"
