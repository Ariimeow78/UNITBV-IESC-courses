#!/usr/bin/env sh
# Afiseaza numele fisierelor binare din directoarele date (recursiv).
usage() {
    echo "Usage: ${0} DIR1 [DIR2 ...]" >&2
    echo "Prints binary files found recursively in given directories." >&2
}

[ "$#" -lt 1 ] && { usage; exit 1; }

for d in "$@"; do
    find "$d" -type f 2>/dev/null | while IFS= read -r f; do
        if LC_ALL=C grep -q '[^[:print:][:space:]]' "$f" 2>/dev/null; then
            echo "$f"
        fi
    done
done
