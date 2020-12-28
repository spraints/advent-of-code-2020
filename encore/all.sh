cargo build --release || exit 1
for n in 1 2 3 4 5 6; do
  echo '------'
  echo day $n
  target/release/adventofcode2020 day${n} < ../fast/in/${n}
done
