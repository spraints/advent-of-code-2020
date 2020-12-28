#!/bin/bash
#/ Usage: ./prep.sh DAYNUMBER

set -o nounset

daynum=${1:-}
if [ "$((daynum + 0))" != "$daynum" ]; then
  cat "$0" | grep ^#/ | cut -c4-
  exit 1
fi

dayfile=src/day${daynum}.rs
if [ -e "$dayfile" ]; then
  echo "$dayfile already exists."
  exit 1
fi

cp src/template.rs $dayfile
git add -N $dayfile

prev=$((daynum - 1))
NL=$'\n'
sed -E -i "" \
  -e "/day${daynum}/d" \
  -e "s/mod day${prev};/mod day${prev};\\${NL}mod day${daynum};/" \
  -e "s/( day${prev}::run.*)/\\1\\${NL}            \"day${daynum}\" => day${daynum}::run(stdin()),/" \
  -e "s/(day${prev}::run,.*)/\\1\\${NL}    time(day${daynum}::run, \"..\\/fast\\/in\\/${daynum}\");/" \
  src/main.rs
