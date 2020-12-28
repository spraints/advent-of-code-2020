#!/bin/bash
cargo build --release || exit 1
for n in $(seq 1 7); do
  echo '------'
  echo day $n
  target/release/adventofcode2020 day${n} < ../fast/in/${n}
done
