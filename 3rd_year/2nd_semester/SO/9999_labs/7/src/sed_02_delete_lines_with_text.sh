#!/usr/bin/env sh
# Sterge liniile care contin textul dat din fisierele specificate.
usage() {
    echo "Usage: ${0} TEXT FILE1 [FILE2 ...]" >&2
    echo "Deletes lines containing TEXT from each file." >&2
}

[ "$#" -lt 2 ] && { usage; exit 1; }

text=$1
shift

for f in "$@"; do
    echo "===== $f ====="
    sed "/$text/d" "$f"
done
