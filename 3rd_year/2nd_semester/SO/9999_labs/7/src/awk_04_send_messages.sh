#!/usr/bin/env sh
# Trimite mesaj studentilor din grupele cerute pe baza fisierului studenti.lst.
usage() {
    echo "Usage: ${0} STUDENTI_LST MESAJ_TXT GRUPA1 [GRUPA2 ...]" >&2
    echo "Sends message to users from requested groups." >&2
}

[ "$#" -lt 3 ] && { usage; exit 1; }

students_file=$1
message_file=$2
shift 2

if [ ! -f "$students_file" ] || [ ! -f "$message_file" ]; then
    echo "Eroare: fisier inexistent" >&2
    exit 1
fi

awk -v groups="$*" '
BEGIN {
    n = split(groups, g, " ")
    for (i = 1; i <= n; i++) wanted[g[i]] = 1
}
{
    # Format: Nume Prenume user grupa
    if (NF >= 4 && ($4 in wanted)) {
        print $3
    }
}' "$students_file" |
while IFS= read -r user; do
    if command -v mail >/dev/null 2>&1; then
        mail -s "Mesaj laborator" "$user" < "$message_file"
        echo "Trimis catre: $user"
    else
        echo "mail indisponibil; destinatar: $user"
    fi
done
