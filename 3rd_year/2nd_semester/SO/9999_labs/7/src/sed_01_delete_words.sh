#!/usr/bin/env sh
# Sterge toate aparitiile cuvintelor date ca parametri din fisierul dat.
usage() {
    echo "Usage: ${0} FILE WORD1 [WORD2 ...]" >&2
    echo "Deletes all occurrences of given words from FILE." >&2
}

[ "$#" -lt 2 ] && { usage; exit 1; }

file=$1
shift

script=''
for w in "$@"; do
    script="${script};s/${w}//g"
done

sed "$script" "$file"
