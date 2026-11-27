# Test the UI with headless software rendering

UI tests run under Xvfb in CI with Mesa's software Vulkan (lavapipe) and `LIBGL_ALWAYS_SOFTWARE=1`, because GitHub runners have no GPU and GPUI renders through wgpu. Locally `scripts/ui-test.sh` falls back to the live display or a headless sway.

**Consequence:** UI tests are slower in CI and need the Mesa packages installed there.
