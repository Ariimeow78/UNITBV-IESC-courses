#!/usr/bin/env sh
# Afiseaza ora curenta in formatul cerut.
usage() {
    echo "Usage: ${0}" >&2
    echo "Prints current time as: ora xx, xx minute, xx secunde." >&2
}

[ "$#" -ne 0 ] && { usage; exit 1; }

date '+%H %M %S' | awk '{printf "ora %s, %s minute, %s secunde\n", $1, $2, $3}'
