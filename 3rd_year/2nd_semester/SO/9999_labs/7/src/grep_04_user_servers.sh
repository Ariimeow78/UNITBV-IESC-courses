#!/usr/bin/env sh
# Afiseaza serverele (host-uri) la care este conectat fiecare utilizator dat.
usage() {
    echo "Usage: ${0} USER1 [USER2 ...]" >&2
    echo "Prints remote servers each user is connected to." >&2
}

[ "$#" -lt 1 ] && { usage; exit 1; }

for u in "$@"; do
    echo "Utilizator: $u"
    who | awk -v usr="$u" '
        $1 == usr {
            host = $NF
            gsub(/[()]/, "", host)
            if (host != "") seen[host] = 1
        }
        END {
            n = 0
            for (h in seen) {
                print "  " h
                n++
            }
            if (n == 0) print "  (niciun server remote detectat)"
        }'
done
