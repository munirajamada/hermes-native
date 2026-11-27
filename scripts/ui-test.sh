#!/usr/bin/env bash
# Run the test suite with a display, so UI tests can create windows.
. "$(dirname -- "$0")/lib.sh"
if command -v xvfb-run >/dev/null 2>&1; then
  exec xvfb-run -a "$CARGO" test --workspace --locked "$@"
fi
if [ -n "${DISPLAY:-}${WAYLAND_DISPLAY:-}" ]; then
  exec "$CARGO" test --workspace --locked "$@"
fi
echo "ui-test: no display and no xvfb-run; install xorg-server-xvfb" >&2
exit 1
