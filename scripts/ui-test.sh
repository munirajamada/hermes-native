#!/usr/bin/env bash
# Run the workspace tests with a display, so UI tests can create windows.
. "$(dirname -- "$0")/lib.sh"

if command -v xvfb-run >/dev/null 2>&1; then
  exec xvfb-run -a "$CARGO" test --workspace --locked "$@"
fi

if [ -n "${DISPLAY:-}${WAYLAND_DISPLAY:-}" ]; then
  exec "$CARGO" test --workspace --locked "$@"
fi

if command -v sway >/dev/null 2>&1; then
  log=$(mktemp)
  WLR_BACKENDS=headless WLR_LIBINPUT_NO_DEVICES=1 WLR_RENDERER=pixman \
    sway -d -c /dev/null >"$log" 2>&1 &
  pid=$!
  trap 'kill "$pid" 2>/dev/null || true; rm -f "$log"' EXIT INT TERM

  display=""
  for _ in $(seq 1 50); do
    display=$(sed -n "s/.*Running compositor on wayland display '\(wayland-[0-9]*\)'.*/\1/p" "$log" | head -1)
    [ -n "$display" ] && break
    sleep 0.1
  done
  if [ -z "$display" ]; then
    echo "ui-test: headless sway did not start; see $log" >&2
    exit 1
  fi

  if WAYLAND_DISPLAY="$display" "$CARGO" test --workspace --locked "$@"; then
    exit 0
  else
    exit 1
  fi
fi

echo "ui-test: no display, xvfb-run, or sway; install one" >&2
exit 1
