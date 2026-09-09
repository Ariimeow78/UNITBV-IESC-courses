#!/usr/bin/env sh
# Sterge toate cifrele din fisierele date.
usage() {
    echo "Usage: ${0} FILE1 [FILE2 ...]" >&2
    echo "Deletes all digits from each file." >&2
}

[ "$#" -lt 1 ] && { usage; exit 1; }

for f in "$@"; do
    echo "===== $f ====="
    sed 's/[0-9]//g' "$f"
done
