# Store state under XDG paths and secrets in the OS keyring

State and logs live under `$XDG_DATA_HOME`/`$XDG_CACHE_HOME` (with `$HERMES_HOME` for gateway logs), and gateway tokens live only in the OS keyring through the `keyring` crate. Writing tokens to a config file would leak them through backups and dotfile repositories.
