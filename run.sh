#!/bin/bash

set -e
bash ./install_dependencies.sh
bash ./format_csv.sh
bash ./postgres.sh

cargo build --release

cargo run --release --bin dspa-source -- --tables ./data/1k-users-sorted/

# Run with default parameters
cargo run --release --bin dspa-mq &
cargo run --release --bin dspa-post-stats > post_stats.log &
cargo run --release --bin dspa-recommendations -- --users 0 1 2 3 4 5 6 7 8 9 > recommendations.log &
cargo run --release --bin dspa-anomalies -- --threshold=3 --sample_size=256 --smoothing=5 > anomalies.log &
cargo run --release --bin dspa-source -- --streams ./data/1k-users-sorted/ --speedup=3600 --delay=3600

# Give the processors time to finish processing
sleep 20

# Clean up processes
pkill dspa-mq
pkill dspa-post-stats
pkill dspa-recommendations
pkill dspa-anomalies
