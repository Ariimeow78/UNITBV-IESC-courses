#!/usr/bin/env sh
# Afiseaza continutul tuturor fisierelor text din director si subdirectoare.
usage() {
    echo "Usage: ${0} DIR" >&2
    echo "Prints content of text files found recursively in DIR." >&2
}

[ "$#" -ne 1 ] && { usage; exit 1; }

find "$1" -type f 2>/dev/null | while IFS= read -r f; do
    if LC_ALL=C grep -q '[^[:print:][:space:]]' "$f" 2>/dev/null; then
        :
    else
        echo "===== $f ====="
        cat "$f"
    fi
done
