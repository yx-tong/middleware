#!/bin/bash

# cd to app root
CWD=$(dirname $0)
if [[ `basename $(pwd)` = 'scripts' ]]; then
    cd ../
else
    cd `dirname $CWD`
fi

docker build -t fc-rust-demo .
docker run --name fc-rust-demo fc-rust-demo bash
docker cp fc-rust-demo:/app/rust-docker-web/target/release/server .
docker rm -f fc-rust-demo