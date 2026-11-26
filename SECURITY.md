# Security Policy

## Supported versions

The project is pre-alpha. There are no supported releases yet; fixes land on
`main`.

## Reporting a vulnerability

Do **not** open a public issue. Use GitHub's private vulnerability reporting:

https://github.com/munirajamada/hermes-native/security/advisories/new

If that is unavailable, email munirajamada@protonmail.com. Include:

- what the issue is and where it lives (crate, file, or flow);
- how to reproduce it;
- the impact you believe it has;
- any suggested fix.

You will get an acknowledgement, and a decision on severity and disclosure
timing.

## Scope

In scope for this client:

- credential handling and token storage;
- the gateway transport (WebSocket and HTTP), including remote headers;
- the embedded webview boundary and navigation policy;
- the terminal and file-pane boundary against the host;
- the update path.

Out of scope here:

- vulnerabilities in the Hermes agent itself — report those upstream at
  https://github.com/NousResearch/hermes-agent;
- issues that require an already-compromised machine;
- denial of service from a deliberately malicious local gateway
  configuration.

## No bounties

There is no paid bounty programme.
