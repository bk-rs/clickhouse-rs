#!/usr/bin/env bash

set -ex

# Prerequire
# ./../clickhouse_tgz_archive/download.sh

# CLICKHOUSE_SERVER_BIN=/bin/clickhouse-server ./tests/run_integration_tests.sh
# RUST_BACKTRACE=full ./tests/run_integration_tests.sh
# RUST_LOG=trace ./tests/run_integration_tests.sh

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

bin_default="${script_path_root}../../clickhouse_tgz_archive/clickhouse/usr/bin/clickhouse-server"
bin="${CLICKHOUSE_SERVER_BIN:-${bin_default}}"

workdir=$(mktemp -d)

mkdir -p "${workdir}/lib"
path="${workdir}/lib/"

mkdir -p "${workdir}/etc"
config_file="${workdir}/etc/config.xml"
tee "${config_file}" <<EOF >/dev/null
<yandex>
    <logger>
        <level>trace</level>
        <console>true</console>
    </logger>

    <http_port>8123</http_port>

    <path>${path}</path>

    <uncompressed_cache_size>8589934592</uncompressed_cache_size>
    <mark_cache_size>5368709120</mark_cache_size>
    <mlock_executable>true</mlock_executable>

    <users>
        <default>
            <password>xxx</password>

            <networks>
                <ip>::/0</ip>
            </networks>

            <profile>default</profile>
            <quota>default</quota>
            <access_management>1</access_management>
        </default>
    </users>

    <profiles>
        <default/>
    </profiles>

    <quotas>
        <default/>
    </quotas>
</yandex>
EOF

mkdir -p "${workdir}/log"
log_file="${workdir}/log/clickhouse-server.log"
errorlog_file="${workdir}/log/clickhouse-server.err.log"

mkdir -p "${workdir}/run"
pid_file="${workdir}/run/clickhouse-server.pid"

# https://unix.stackexchange.com/questions/55913/whats-the-easiest-way-to-find-an-unused-local-port
read LOWERPORT UPPERPORT < /proc/sys/net/ipv4/ip_local_port_range
http_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)

cleanup() {
  test -f "${pid_file}" && kill $(cat "${pid_file}")
  test -f "${errorlog_file}" && (cat "${errorlog_file}" | grep -v 'Connection reset by peer' | grep -v 'Broken pipe')
  rm -rf "${workdir}"
}
trap cleanup EXIT

$(${bin} --config-file="${config_file}" --log-file="${log_file}" --errorlog-file="${errorlog_file}" --pid-file="${pid_file}" --daemon -- --path="${path}" --http_port=${http_port})

sleep 2

export CLICKHOUSE_HTTP_URL="http://127.0.0.1:${http_port}"

cargo test -p clickhouse-http-client --features _integration_tests -- --nocapture
