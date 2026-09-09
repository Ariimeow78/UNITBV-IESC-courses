#!/usr/bin/env sh
# Sterge dintre primele 30 linii pe cele care contin textul dat.
usage() {
    echo "Usage: ${0} TEXT FILE1 [FILE2 ...]" >&2
    echo "Deletes matching TEXT lines only within first 30 lines." >&2
}

[ "$#" -lt 2 ] && { usage; exit 1; }

text=$1
shift

for f in "$@"; do
    echo "===== $f ====="
    sed "1,30{/$text/d;}" "$f"
done
