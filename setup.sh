#!/bin/bash

set -e

bash ./install_dependencies.sh
bash ./format_csv.sh
bash ./postgres.sh

cargo run --bin dspa-source -- --tables ./data/1k-users-sorted/

# Run with default parameters
cargo run --bin dspa-mq &
PID_MQ=$!

cargo run --bin dspa-post-stats > post_stats.log & 
PID_POST_STATS=$!

cargo run --bin dspa-recommendations -- --users 0 1 2 3 4 5 6 7 8 9 > recommendations.log &
PID_RECOMMENDATIONS=$!

cargo run --bin dspa-anomalies > anomalies.log &
PID_ANOMALIES=$!

cargo run --bin -- --streams ./data/1k-users-sorted/

sleep 20

kill $PID_POST_STATS
kill $PID_RECOMMENDATIONS
kill $PID_ANOMALIES
kill $PID_MQ