#!/usr/bin/env bash

. tests/helper.sh
setup

# Should initalize hooky
hooky init --no-pre-commit &>/dev/null

[ ! -d .hooky ] && error "Should initialize hooky: Failed to initialize hooky"

ok "Should initialize hooky"

# Should add a commit-msg hook
hooky add commit-msg &>/dev/null

[ -f .git/hooks/commit-msg ] && error "Should add a commit-msg hook: Failed to add commit-msg hook" && exit 1

ok "Should add a commit-msg hook"
