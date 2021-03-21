#!/usr/bin/env bash

set -ex

# Prerequire
# ./download.sh

# ./gen_format_files.sh

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

bin_server="${script_path_root}clickhouse/usr/bin/clickhouse-server"
bin_client="${script_path_root}clickhouse/usr/bin/clickhouse-client"

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

query=$(cat <<-END
SELECT
    array(1, 2) AS array1,
    array('a', 'b') AS array2,
    tuple(1, 'a') AS tuple1,
    tuple(1, NULL) AS tuple2,
    CAST((['1', '2', '3'], ['Ready', 'Steady', 'Go']), 'Map(String, String)') AS map1
END
)

files_dir="${script_path_root}../clickhouse-format/tests/files/"

formats=("JSON" "JSONStrings" "JSONCompact" "JSONCompactStrings")
for format in ${formats[*]}; do
    $(echo ${query} FORMAT ${format} | ${bin_client} --allow_experimental_map_type 1 --password xxx | python3 -m json.tool > "${files_dir}${format}.json")
done

formats=("TSV" "TSVRaw" "TSVWithNames" "TSVWithNamesAndTypes")
for format in ${formats[*]}; do
    $(echo ${query} FORMAT ${format} | ${bin_client} --allow_experimental_map_type 1 --password xxx > "${files_dir}${format}.tsv")
done

formats=("CSV" "CSVWithNames")
for format in ${formats[*]}; do
    $(echo ${query} FORMAT ${format} | ${bin_client} --allow_experimental_map_type 1 --password xxx > "${files_dir}${format}.csv")
done

sleep 1
