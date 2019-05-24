#!/bin/bash
set -e

# Make data dir
if [[ ! -d "data" ]]; then
    mkdir data
fi

# Download data

## 1k data
if [[ ! -d "data/1k-users-sorted" ]]; then
    curl -o temp.zip "https://polybox.ethz.ch/index.php/s/qRlRpFhoPtdO6bR/download"
    unzip temp -d data
    rm temp.zip

    ## Blacklist
    curl -o temp.gz "https://polybox.ethz.ch/index.php/s/spm6LcGA7olA0AF/download"
    gunzip -c temp.gz > data/1k-users-sorted/streams/comment_blacklist.csv
    rm temp.gz

    # Convert from MACROMAN to UTF-8 format
    find "data/1k-users-sorted/" -name "*.csv" -exec bash -c 'iconv -f MACROMAN -t UTF8 $0 > "$0.temp" && rm $0 && mv "$0.temp" $0' {} \;
fi

## 10k data
if [[ ! -d "data/10k-users-sorted" ]]; then
    curl -o temp.zip "https://polybox.ethz.ch/index.php/s/8JRHOc3fICXtqzN/download"
    unzip temp -d data
    rm temp.zip

    ## Blacklist
    curl -o temp.gz "https://polybox.ethz.ch/index.php/s/eeWnen4eH7D96xN/download"
    gunzip -c temp.gz > data/10k-users-sorted/streams/comment_blacklist.csv
    rm temp.gz

    # Convert from MACROMAN to UTF-8 format
    find "data/10k-users-sorted/" -name "*.csv" -exec bash -c 'iconv -f MACROMAN -t UTF8 $0 > "$0.temp" && rm $0 && mv "$0.temp" $0' {} \;
fi

# # Make csv headers unique
find . -name "tagclass_isSubclassOf_tagclass.csv" -exec sed -i '' "1s/TagClass\.id\|TagClass\.id/TagClass\.id\|Parent\.id/" {} \;
find . -name "place_isPartOf_place.csv" -exec sed -i '' "1s/Place\.id\|Place\.id/Place\.id\|Parent\.id/" {} \;
find . -name "person_knows_person.csv" -exec sed -i '' "1s/Person\.id\|Person\.id/Person\.id\|Acquaintance\.id/" {} \;