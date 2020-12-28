#!/bin/bash
[ -z "$TIMER" ] && (cargo build --release || exit 1)
for n in $(seq 1 25); do
  echo '------'
  echo day $n
  target/release/adventofcode2020 day${n} < ../fast/in/${n} || break
done
