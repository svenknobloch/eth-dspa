#!/bin/bash
set -e

export DATABASE_URL="postgres://root@localhost/dspa"

docker run --rm --name postgres -p 5432:5432 -e POSTGRES_USER="root" -e POSTGRES_PASSWORD="" -d postgres:11-alpine

sleep 5

cd dspa-lib

diesel database setup
