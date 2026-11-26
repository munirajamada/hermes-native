# Shared setup. Sourced by the other scripts; not run directly.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

CARGO=${CARGO:-cargo}
