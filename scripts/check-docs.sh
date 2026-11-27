#!/usr/bin/env bash
. "$(dirname -- "$0")/lib.sh"
exec python3 "$ROOT/scripts/check_docs.py"
