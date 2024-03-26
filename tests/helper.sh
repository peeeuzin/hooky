file=$(basename "$0")

setup() {
    set -o errexit
    set -o nounset
    set -o pipefail

    cd $(mktemp -d)
    git init &>/dev/null
}

error() {
    echo -e "$file: \e[0;31mERROR:\e[m $1"
    exit 1
}

ok() {
    echo -e "$file: \e[0;32mOK\e[m $1"
}
