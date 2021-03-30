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

cmd="${bin_server} --config-file="${config_file}" --log-file="${log_file}" --errorlog-file="${errorlog_file}" --pid-file="${pid_file}" --daemon -- --path="${path}" --tcp_port=${tcp_port}"
$(${cmd})

sleep 2

query_create_table=$(cat <<-END
CREATE TABLE t_testing_format
(
    array1 Array(UInt8),
    array2 Array(String),
    tuple1 Tuple(UInt8, String),
    tuple2 Tuple(UInt8, Nullable(String)),
    map1 Map(String, String)
) ENGINE=Memory
END
)
$(echo ${query_create_table} | ${bin_client} --allow_experimental_map_type 1 --port ${tcp_port} --password xxx)

query_insert=$(cat <<-END
INSERT INTO t_testing_format VALUES 
    ([1, 2], ['a', 'b'], (1, 'a'), (1, null), {'1':'Ready', '2':'Steady', '3':'Go'}), 
    ([3, 4], ['c', 'd'], (2, 'b'), (2, 'b'), {})
END
)
$(echo ${query_insert} | ${bin_client} --allow_experimental_map_type 1 --port ${tcp_port} --password xxx)

query_select=$(cat <<-END
SELECT array1, array2, tuple1, tuple2, map1 FROM t_testing_format
END
)
files_path="${script_path_root}files"

formats=("JSON" "JSONStrings" "JSONCompact" "JSONCompactStrings")
for format in ${formats[*]}; do
    $(echo ${query_select} FORMAT ${format} | ${bin_client} --allow_experimental_map_type 1 --port ${tcp_port} --password xxx | python3 -m json.tool > "${files_path}/${format}.json")
done

formats=("TSV" "TSVRaw" "TSVWithNames" "TSVWithNamesAndTypes")
for format in ${formats[*]}; do
    $(echo ${query_select} FORMAT ${format} | ${bin_client} --allow_experimental_map_type 1 --port ${tcp_port} --password xxx > "${files_path}/${format}.tsv")
done

# SKIP, because don't support tuple
# formats=("CSV" "CSVWithNames")
# for format in ${formats[*]}; do
#     $(echo ${query_select} FORMAT ${format} | ${bin_client} --allow_experimental_map_type 1 --format_csv_delimiter '|' --port ${tcp_port} --password xxx > "${files_path}/${format}.csv")
# done

formats=("JSONEachRow" "JSONStringsEachRow" "JSONCompactEachRow" "JSONCompactStringsEachRow" "JSONEachRowWithProgress" "JSONStringsEachRowWithProgress" "JSONCompactEachRowWithNamesAndTypes" "JSONCompactStringsEachRowWithNamesAndTypes")
for format in ${formats[*]}; do
    $(echo ${query_select} FORMAT ${format} | ${bin_client} --allow_experimental_map_type 1 --port ${tcp_port} --password xxx > "${files_path}/${format}.txt")
done

sleep 1
