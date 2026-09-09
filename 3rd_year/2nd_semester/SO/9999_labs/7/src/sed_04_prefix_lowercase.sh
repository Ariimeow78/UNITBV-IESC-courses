#!/usr/bin/env sh
# Adauga in fata fiecarei litere mici cuvantul dat.
usage() {
    echo "Usage: ${0} PREFIX FILE1 [FILE2 ...]" >&2
    echo "Prefixes each lowercase letter with PREFIX in each file." >&2
}

[ "$#" -lt 2 ] && { usage; exit 1; }

prefix=$1
shift

for f in "$@"; do
    echo "===== $f ====="
    sed "s/[a-z]/${prefix}&/g" "$f"
done
