# DSPA Semester Project

Data Stream Processing and Analytics Course, offered by Vasiliki Kalavri, Zaheer Chothia and Michal Wawrzoniak.

Project Repository by Sven Knobloch and David Yenicelik.

## Quick Start

Requirements: `brew`, `cargo`, `docker`

For Mac OSX:
```
git clone git@gitlab.ethz.ch:knsven/dspa-semester-project.git
cd ./dspa-semester-project/
./run.sh
```

For other Unix Systems (not tested):
```
git clone git@gitlab.ethz.ch:knsven/dspa-semester-project.git
cd ./dspa-semester-project/
-> Install zeromq
cargo install diesel_cli --no-default-features --features="postgres"
./format_csv.sh
./postgres.sh

# Pulled from run.sh

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
```

Does not run on Windows!

Output will be found in `post_stats.log`, `recommendations.log` and `anomalies.log`

For more details, see below

## Contents

### dspa-lib
Contains common functionality for the other crates in the workspace.

#### migrations
Contains the migrations for the PostgreSQL database. Can be applied using the `diesel_cli`

#### `lib.rs`
General common types and constants, including `MAX_DELAY` and socket/database addresses.

#### `schema` module
Contains bindings for the database. Auto generated by `diesel`.

#### `operators` module
Contains common operators:
* `Source` - Operator that initializes the receiving side for Post/Comment/Like streams
* `Order` - Operator that blocks events until all dependent events have arrived.
    * Likes are released as soon as the corresponding post has arrived
    * Comments are released as soon as the root post has arrived

#### `records` module
Contains data types for all stream and table records.

### dspa-source
Contains functionality for reading records from csv and then inserting them into the database or event stream.

#### `operators` module
Contains source operators:
* `BoundedDelay` - delay all records by a random amount between 0 and the given bound
* `Insert` - insert a given table record into the database
* `Publish` - Send a given record to the source socket

#### **Usage**
Required
* `path` - path to the directory containing the streams and tables directories

Options
* `--tables` - read table records into the database
* `--streams` - read stream records into database and event stream

### dspa-mq
Basic message broker. Receives input from the source socket, sets the appropriate topic and then forwards the records to all subscribed listeners.

#### **Usage**
Takes no special arguments

### dspa-post-stats
Contains functionality for task 1.

#### `operators` module
Contains active post operators:
* `PostStats` - Maintains a list of new or currently active posts by post id, based on comment or like activity
    * Ignores posts that lost active status
    * No output is generated for times that do not have any active posts to make the output more readable

#### **Usage**
Takes no special arguments

### dspa-recommendations
Contains functionality for task 2.

#### `operators` module
Contains recommendation operators:
* `Window` - maintains a window of all activity over the past `size` hours and outputs it at a rate of `frequency`
* `Recommendations` - takes a list of events and computes a list of friend recommendations for each entry in `users`
    * The recommendations are based on common interactions between the selected users and all other users on posts
    * No output is generated for times that do not have any recommendations
    * The list of recommendations for each user is limited to five but may be less if not enough matches are found

#### **Usage**
Options
* `--users` - takes a sequence of user ids to make recommendations for

### dspa-anomalies

#### `statistics` module
Contains helper structs for statistics.

* `RollingStatistic` - calculates mean and variance for window of samples of a given size
* `OnlineStatistic` - calculates mean and variance for a continuous stream of samples with a minumum size

#### `operators` module
Contains anomaly operators:
* `Anomalies` - calculates anomaly values for unique words per word for posts, unique words per word for comments and number of tags based on the given threshold and smoothing
    * Threshold defines number of standard deviations a value must be away to be considered an anomaly. Higher values are more tolerant.
    * (Laplace) Smoothing helps account for short posts that have a high or perfect unique word per word ratio. Higher values are more tolerant.

#### **Usage**
Options
* `--smoothing` - set the smoothing parameter (default: 3)
* `--sample_size` - set the minimum sample size (default: 256)
* `--threshold` - set the standard deviation threshold (default: 3)

### Scripts
Scripts used to run the project. The development is done on OSX and the scripts are therefore layed out for OSX. If running on a different unix system, the scripts will not run properly. `brew`, `cargo`, `rust` and `docker` are assumed to be present on the system.

#### `install_dependencies.sh`
Installs dependencies, namely `zeromq` and `diesel_cli`.

#### `format_csv.sh`
This script downloads the data files and renames them, fixes CSV headers and converts the encoding such that the programs can properly read them.

#### `postgres.sh`
This script kills and then creates a PostgreSQL container using `Docker` and then runs migrations to define the database schema.

#### `run.sh`
This script combines the previous ones and then runs the project. When running the project more than once, the setup portion of the script does not need to be run again since it takes some time to populate the database and download the data, etc. The binaries that are executed are given some default parameters and are all piped into separate log files.