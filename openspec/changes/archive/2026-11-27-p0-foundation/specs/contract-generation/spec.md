## Purpose

Defines the Rust types and method inventory the client uses to speak to the Hermes gateway, generated from the vendored OpenRPC document.

## ADDED Requirements

### Requirement: Generated contract types

The client SHALL obtain every gateway method, notification, and server request from the vendored OpenRPC document, and SHALL NOT hand-write wire types.

#### Scenario: Regeneration is deterministic

- **WHEN** the generator runs twice on the same document
- **THEN** it writes byte-identical output
- **AND** every method name in the document appears in the generated inventory

#### Scenario: Drift fails the check

- **WHEN** the committed generated file differs from a fresh generation
- **THEN** the CI check fails and names the file
