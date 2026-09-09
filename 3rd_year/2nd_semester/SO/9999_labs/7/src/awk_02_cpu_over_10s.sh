#!/usr/bin/env sh
# Afiseaza procesele cu timp CPU total > 10 secunde.
usage() {
    echo "Usage: ${0}" >&2
    echo "Lists processes with total CPU time greater than 10 seconds." >&2
}

[ "$#" -ne 0 ] && { usage; exit 1; }

ps -e -o pid= -o user= -o comm= -o time= | awk '
function to_seconds(t, n,a,s) {
    n = split(t, a, ":")
    if (n == 2) {
        return a[1] * 60 + a[2]
    }
    if (n == 3) {
        return a[1] * 3600 + a[2] * 60 + a[3]
    }
    return 0
}
{
    sec = to_seconds($NF)
    if (sec > 10) {
        printf "PID=%s USER=%s CMD=%s CPU=%s (%ds)\n", $1, $2, $3, $NF, sec
    }
}'
