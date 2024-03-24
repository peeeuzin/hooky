#!/usr/bin/env bash

. tests/helper.sh
setup

# Should initalize hooky
hooky init &>/dev/null

[ ! -d .hooky ] && error "Failed to initialize hooky"

ok "Should initialize hooky"

# Should uninstall hooky
hooky uninstall &>/dev/null

[ -d .hooky ] && error "Failed to uninstall hooky"

ok "Should uninstall hooky"
