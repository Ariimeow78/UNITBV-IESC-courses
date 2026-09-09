#!/usr/bin/env sh
# Listeaza cele mai mari 10 fisiere dintr-un director dat.
usage() {
    echo "Usage: ${0} DIR" >&2
    echo "Lists top 10 largest files recursively in DIR." >&2
}

[ "$#" -ne 1 ] && { usage; exit 1; }

find "$1" -type f -exec ls -ln {} + 2>/dev/null |
awk '{print $5, $9}' |
sort -nr |
awk 'NR <= 10 {printf "%s bytes\t%s\n", $1, $2}'
