# Design Document (knsven, yedavid)
Source: [https://gitlab.ethz.ch/knsven/dspa-semester-project](https://gitlab.ethz.ch/knsven/dspa-semester-project)

## Overview

![architecture](architecture.jpg)

This project will be written using the Rust language and the `timely-dataflow` stream processing library along with some scripting in either Bash or Python for tooling/setup. Rust was chosen since it is the more performant than Java/Flink. The project will consist of multiple crates, one for each subtask and some additional supporting crates.

* `dspa-lib` - Common types for all the crates
* `dspa-source` - Data preparation
* `dspa-mq` - Message Broker
* `dspa-post-stats` - Active Post Statistics
* `dspa-recommendations` - Recommendations
* `dspa-anomalies` - Unusual Activity Detection

## Data Preparation
Data preparation will be done in two parts. The first part will extract the static information from the given source files and insert them into a structured Postgres database. The second part will extract the stream data from the files and send it to the message broker for distribution, at scaled time intervals.

## Message Broker
The message broker is responsible for managing the incoming message streams and distributing them to interested parties and will be implemented using [ØMQ](http://zeromq.org). The message broker will provide a set of topics that can be subscribed to by connected clients. These messages will be passed using ØMQ's publish/subscribe sockets.

## Active Post Statistics

### Overview
The active post statistics will be calculated using a stream processor. It will subscribe to the message broker to recieve notifications on topics and then keep track of the aggregated data using a sliding window that dumps the statistics every thirty minutes for comments and replies, and every hour for unique user count. These results will be output to a log.

### Input
Static: N/A  
Streams: Posts, Comments, Likes

### Output
Every 30 minutes: Logs containing a list of active posts, each with an associated comment and reply number.
Every 60 minutes: Logs containing a list of active posts, each with an associated unique user count.

## Recommendations

### Overview
The recommendations will also be calculated using a stream processor. It will utilize the static data from the database as well as the stream data which it will receive from the message broker. Then it will make recommendations based on a similarity metric that compares the data of the user and the user's friends with other users and their friends. These results will be output to a log.

### Input
Static: Organizations, Friends, Forums  
Streams: Posts, Comments, Likes

### Output
Every 60 minutes: Logs containing 

## Unusual Activity Detection
The unusual activity detection will also be done using a stream processor. It will again utilize both static data, like forums and friends, as well as streamed data, like posts, comments, replies and likes. An initial baseline profile of the user will be constructed using the first couple of datapoints for each user and the following points will be checked for unusual activity using an online clustering algorithm.

## Environment
Each stream processor will be treated as an individual server and run in a virtualized environment like Docker or Kubernetes to simulate a real world situation with networking.

## External Tools
* Scripting Language (Python/Bash)
* Virtualized Environment (Docker/Kubernetes)
* Message Broker (ØMQ)