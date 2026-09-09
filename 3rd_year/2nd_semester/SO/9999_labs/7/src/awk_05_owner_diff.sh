#!/usr/bin/env sh
# Afiseaza fisierele al caror proprietar difera de proprietarul directorului parinte.
usage() {
    echo "Usage: ${0} DIR" >&2
    echo "Lists files whose owner differs from parent directory owner." >&2
}

[ "$#" -ne 1 ] && { usage; exit 1; }

find "$1" -type f 2>/dev/null | while IFS= read -r f; do
    d=$(dirname "$f")
    f_owner=$(ls -ldn "$f" | awk '{print $3}')
    d_owner=$(ls -ldn "$d" | awk '{print $3}')
    if [ "$f_owner" != "$d_owner" ]; then
        printf "%s (owner file=%s, owner dir=%s)\n" "$f" "$f_owner" "$d_owner"
    fi
done
