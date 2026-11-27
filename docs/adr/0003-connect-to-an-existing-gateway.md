# Connect to an existing Hermes gateway

The client never bundles the Python agent. It attaches to a local gateway with `hermes gateway attach --json` (URL, single-use ticket, subprotocols) or a saved remote connection. Bundling the agent would make every release a Python packaging project and duplicate upstream updates.
