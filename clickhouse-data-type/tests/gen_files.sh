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

query_int_uint=$(cat <<-END
SELECT
    toTypeName(toUInt8(0)) as uint8,
    toTypeName(toUInt16(0)) as uint16,
    toTypeName(toUInt32(0)) as uint32,
    toTypeName(toUInt64(0)) as uint64,
    toTypeName(toUInt256(0)) as uint256,
    toTypeName(toInt8(0)) as int8,
    toTypeName(toInt16(0)) as int16,
    toTypeName(toInt32(0)) as int32,
    toTypeName(toInt64(0)) as int64,
    toTypeName(toInt128(0)) as int128,
    toTypeName(toInt256(0)) as int256
END
)
$(echo ${query_int_uint} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/int_uint.txt")


sleep 1
