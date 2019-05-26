# Midterm Progress Report - knsven, yedavid

## Progress

### Scripts

* `format_csv.sh` - This script downloads and formats the data from the polybox server. It also converts the data from MACROMAN encoding to UTF-8 so that it can be properly parsed by Rust programs. Finally it makes the csv headers unique so that they can be parsed properly.
* `postgres.sh` - This script launches the database as a Docker container and runs migrations to intialize the tables. The database holds both static data as well as streaming data so that the stream processors can safely discard data to save memory and load it back if required later.

### `dspa-lib`

#### `records` module
The `records` module contains the structs used for data representation. Each struct implements the `serde` traits `Serialize` and `Deserialize` to automatically derive serialization and deserialization from csv and network representations. Additionally, each record maps to their respective table using the `diesel` ORM. This makes inserting and fetching records from the database trivial.

#### `operators` module
The `operator` module contains common operators that can be utilized across crates. The `StreamDataSource` struct can be converted to a stream source of data given a data type to listen for. Additionally, the `Ordered` trait allows for reordering of stream data to make sure that it appears in order, specifically for likes and comment heirarchies.

### `dspa-mq`
The `dspa-mq` crate contains the message queue implementation using `zmq`. This implementation was relatively simple since `zmq` provides PUB/SUB socket types and all that the crate does is forward all data from `dspa-source` to all listeners on the respective topics.

### `dspa-source`
The `dspa-source` crate provides a cli for streaming data from the csvs. It has the option to populate the database as well as play the stream information. It also takes command line arguments for the speedup factor and delay. The record playback is currently being rewritten as a Timely application instead of the current process of reading and outputting the csvs to help simplify the dataflow and merging/interleaving of records. 

### `dspa-post-stats`
The `dspa-post-stats` crate contains the active post statistics generation. It is currently not completely finished but the preliminary implementations for accumulating and displaying the active posts is present. As it stands, the stream state manages the incoming comments and likes in an unordered manner to then wait for the respective post. If the post or comment parent has already arrived, the events are immediately handled. Currently the posts are discarded after the become inactive and future events are ignored.

## Divergences
The structure of the project has not changed much from the plan so far. The only real difference is that the stream data is now also stored in the database. It is expected that there might be some changes that need to be made to accomodate the more complex reccomendations and anomaly detection but these are currently not implemented.

## Challenges
So far the tasks have been pretty managable. There were some slight hiccups that cost time to figure out, including having non UTF-8 encoded data, out of order data (comments and likes having timestamps that predate their respective posts) and general debugging along the way. Timely has been very accomodating and simple to work with.

## Outstanding Tasks
Most of the work for the parsing and streaming is done which leaves the remainder of the time for the recommendations, anomalies and final testing and evaluation. This leaves approximately one week per task as the target until the final deadline.

### `dspa-post-stats`
Currently, the post statistics do not resume after a post has become inactive. This behavior will be modified to resume active status by storing and then later loading the relevant information from the database, if time permits. The finalized version should be finished by the end of the week.

### `dspa-recommendations`
We enumerated all possible configurations on a famous movie dataset that is used to benchmark recommender system algorithms, and have concluded that we will use kNN (screenshot of first formula [here](https://surprise.readthedocs.io/en/stable/knn_inspired.html)) to predict unseen items for a user X. kNN requires a similarity metric, and we will use the default msd similarity (screenshot first formula [here](https://surprise.readthedocs.io/en/stable/similarities.html)) to score similarity between any two items in the dataset. including all possible posts will result in a huge vector. We plan to only use posts and likes within certain time windows, and possibly random sampling of posts to solve this issue.

### `dspa-anomalies`
We have not looked too deeply into the anomaly detection yet, but plan to use an online SVM to create lower-dimensional vector embeddings which define a generative gaussian mixture models. This generate model can then be used to determine if certain behavior is outside this model, and thus defined as an outlier. This aspect will require more experimentation beforehand, however.

### Testing & Overall Coherency
Currently there are no tests to help verify the correctness of the implementations. These are planned to be added but the actual implementation takes priority.