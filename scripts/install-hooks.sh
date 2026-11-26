#!/usr/bin/env bash
. "$(dirname -- "$0")/lib.sh"
git config core.hooksPath .githooks
chmod +x .githooks/*
echo "hooks installed: $(git config core.hooksPath)"
