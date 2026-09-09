#!/usr/bin/env sh
# Afiseaza utilizatorii care au GID-ul dat si numele grupului.
usage() {
    echo "Usage: ${0} GID" >&2
    echo "Prints users belonging to group with the specified GID." >&2
}

[ "$#" -ne 1 ] && { usage; exit 1; }

gid=$1
group_name=$(grep "^[^:]*:[^:]*:${gid}:" /etc/group | awk -F: 'NR==1{print $1}')

if [ -z "$group_name" ]; then
    echo "Nu exista grup cu GID=$gid"
    exit 1
fi

echo "Grup: $group_name (GID=$gid)"
awk -F: -v gid="$gid" '$4 == gid {print $1}' /etc/passwd
