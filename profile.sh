target/release/seedcracker &
pid=$!
sample $pid 30 -file sample.txt
inferno-collapse-sample sample.txt > stacks.folded
cat stacks.folded | inferno-flamegraph > flamegraph.svg
kill $pid