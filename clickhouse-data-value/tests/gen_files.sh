#!/usr/bin/env bash

set -ex

# Prerequire
# ./../clickhouse_tgz_archive/download.sh

# ./tests/gen_files.sh

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

bin_server="${script_path_root}../../clickhouse_tgz_archive/clickhouse/usr/bin/clickhouse-server"
bin_client="${script_path_root}../../clickhouse_tgz_archive/clickhouse/usr/bin/clickhouse-client"

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

    <tcp_port>9000</tcp_port>

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
tcp_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)

cleanup() {
  test -f "${pid_file}" && kill $(cat "${pid_file}")
  test -f "${errorlog_file}" && (cat "${errorlog_file}" | grep -v 'Connection reset by peer' | grep -v 'Broken pipe')
  rm -rf "${workdir}"
}
trap cleanup EXIT

$(${bin_server} --config-file="${config_file}" --log-file="${log_file}" --errorlog-file="${errorlog_file}" --pid-file="${pid_file}" --daemon -- --path="${path}" --tcp_port=${tcp_port})

sleep 2

files_path="${script_path_root}files"

query_date=$(cat <<-END
SELECT
    toDate('2021-03-01') as date
END
)
$(echo ${query_date} FORMAT JSONEachRow | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/date.txt")


query_datetime=$(cat <<-END
SELECT
    toDateTime('2021-03-01 01:02:03', 'UTC') as datetime_utc,
    toDateTime('2021-03-01 01:02:03', 'Asia/Shanghai') as datetime_shanghai
END
)
$(echo ${query_datetime} FORMAT JSONEachRow | ${bin_client} --date_time_output_format simple --port ${tcp_port} --password xxx > "${files_path}/datetime_simple.txt")
$(echo ${query_datetime} FORMAT JSONEachRow | ${bin_client} --date_time_output_format iso --port ${tcp_port} --password xxx > "${files_path}/datetime_iso.txt")
$(echo ${query_datetime} FORMAT JSONEachRow | ${bin_client} --date_time_output_format unix_timestamp --port ${tcp_port} --password xxx > "${files_path}/datetime_unix_timestamp.txt")

query_datetime64=$(cat <<-END
SELECT
    toDateTime64('2021-03-01 01:02:03.123456789', 3, 'UTC') as datetime64_milli_utc,
    toDateTime('2021-03-01 01:02:03.123456789', 3, 'Asia/Shanghai') as datetime64_milli_shanghai,
    toDateTime64('2021-03-01 01:02:03.123456789', 6, 'UTC') as datetime64_micro_utc,
    toDateTime('2021-03-01 01:02:03.123456789', 6, 'Asia/Shanghai') as datetime64_micro_shanghai,
    toDateTime64('2021-03-01 01:02:03.123456789', 9, 'UTC') as datetime64_nano_utc,
    toDateTime('2021-03-01 01:02:03.123456789', 9, 'Asia/Shanghai') as datetime64_nano_shanghai
END
)
$(echo ${query_datetime64} FORMAT JSONEachRow | ${bin_client} --date_time_output_format simple --port ${tcp_port} --password xxx > "${files_path}/datetime64_simple.txt")
$(echo ${query_datetime64} FORMAT JSONEachRow | ${bin_client} --date_time_output_format iso --port ${tcp_port} --password xxx > "${files_path}/datetime64_iso.txt")
$(echo ${query_datetime64} FORMAT JSONEachRow | ${bin_client} --date_time_output_format unix_timestamp --port ${tcp_port} --password xxx > "${files_path}/datetime64_unix_timestamp.txt")


sleep 1
