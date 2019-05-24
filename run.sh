#!/bin/bash

set -e
bash ./install_dependencies.sh
bash ./format_csv.sh
bash ./postgres.sh

cargo build --release

cargo run --release --bin dspa-source -- --tables ./data/1k-users-sorted/

# Run with default parameters
cargo run --release --bin dspa-mq &
PID_MQ=$!

cargo run --release --bin dspa-post-stats > post_stats.log & 
PID_POST_STATS=$!

cargo run --release --bin dspa-recommendations -- --users 0 1 2 3 4 5 6 7 8 9 > recommendations.log &
PID_RECOMMENDATIONS=$!

cargo run --release --bin dspa-anomalies -- --threshold=0 > anomalies.log &
PID_ANOMALIES=$!

cargo run --release --bin dspa-source -- --streams ./data/1k-users-sorted/ --speedup=3600 --delay=3600

sleep 20

kill $PID_POST_STATS
kill $PID_RECOMMENDATIONS
kill $PID_ANOMALIES
kill $PID_MQ
