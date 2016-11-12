#!/bin/bash

set -eux

data_url="https://gist.githubusercontent.com/anonymous/0897526852f1e8e22c8d5182186dbed7/raw/01fc92acf347eebd895c39d5c31631360621113f/xaa"
pattern="02/Jul/1995:16:30:08"

if [ ! -f bench_data ]; then
	wget $data_url
	mv xaa bench_data
fi

time cargo run bench_data $pattern
