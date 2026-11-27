## Purpose

Defines the first native window: the product identity, the GPUI stack initialization, and the display requirement.

## ADDED Requirements

### Requirement: Single native window

The client SHALL open exactly one themed window on startup with the product name and an application id, on X11 or XWayland.

#### Scenario: Window opens on X11

- **WHEN** the application starts with an X11 display
- **THEN** a window appears with the product title and app id

#### Scenario: No display

- **WHEN** neither an X11 nor a Wayland display is available
- **THEN** the application exits with an actionable error
