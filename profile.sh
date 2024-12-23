target/release/seedcracker &
pid=$!
sample $pid 30 -file sample.txt
inferno-collapse-sample sample.txt > stacks.folded
inferno-flamegraph stacks.folded > flamegraph.svg