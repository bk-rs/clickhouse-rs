#!/usr/bin/env bash

set -ex

# ./download.sh v21.4.1.6378-testing

tag="${1:-v21.2.7.11-stable}"
version=`echo $tag | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+'`

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

rm -rf "${script_path_root}clickhouse-common-static-${version}.tgz"
rm -rf "${script_path_root}clickhouse-server-${version}.tgz"
rm -rf "${script_path_root}clickhouse-client-${version}.tgz"
wget -O "${script_path_root}clickhouse-common-static-${version}.tgz" "https://github.com/ClickHouse/ClickHouse/releases/download/${tag}/clickhouse-common-static-${version}.tgz"
wget -O "${script_path_root}clickhouse-server-${version}.tgz" "https://github.com/ClickHouse/ClickHouse/releases/download/${tag}/clickhouse-server-${version}.tgz"
wget -O "${script_path_root}clickhouse-client-${version}.tgz" "https://github.com/ClickHouse/ClickHouse/releases/download/${tag}/clickhouse-client-${version}.tgz"

tgz_root_path="${script_path_root}clickhouse"
rm -rf "${tgz_root_path}"
mkdir "${tgz_root_path}"
tar -zxvf "${script_path_root}clickhouse-common-static-${version}.tgz" -C "${tgz_root_path}" --strip-components=2
tar -zxvf "${script_path_root}clickhouse-server-${version}.tgz" -C "${tgz_root_path}" --strip-components=2
tar -zxvf "${script_path_root}clickhouse-client-${version}.tgz" -C "${tgz_root_path}" --strip-components=2
