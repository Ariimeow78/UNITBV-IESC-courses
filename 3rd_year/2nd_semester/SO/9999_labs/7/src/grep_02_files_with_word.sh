#!/usr/bin/env sh
# Afiseaza fisierele din linia de comanda care contin cuvantul dat + numarul lor.
usage() {
    echo "Usage: ${0} WORD FILE1 [FILE2 ...]" >&2
    echo "Prints file names containing WORD and their count." >&2
}

[ "$#" -lt 2 ] && { usage; exit 1; }

word=$1
shift

matches=$(grep -l -- "$word" "$@" 2>/dev/null || true)
[ -n "$matches" ] && printf "%s\n" "$matches"
count=$(printf "%s\n" "$matches" | awk 'NF{c++} END{print c+0}')
echo "Numar fisiere: $count"
