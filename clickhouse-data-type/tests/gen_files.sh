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

# 
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

# 
query_float=$(cat <<-END
SELECT
    toTypeName(toFloat32(0.0)) as float32,
    toTypeName(toFloat64(0.0)) as float64
END
)
$(echo ${query_float} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/float.txt")

# 
query_decimal_create_table=$(cat <<-END
CREATE TABLE t_testing_type_decimal
(
    f_decimal32 Decimal32(9),
    f_decimal64 Decimal64(18),
    f_decimal128 Decimal128(38),
    f_decimal256 Decimal256(76)
) ENGINE=Memory
END
)
$(echo ${query_decimal_create_table} | ${bin_client} --allow_experimental_bigint_types 1 --port ${tcp_port} --password xxx)

query_decimal_insert=$(cat <<-END
INSERT INTO t_testing_type_decimal VALUES 
    (0.0, 0.0, 0.0, 0.0)
END
)
$(echo ${query_decimal_insert} | ${bin_client} --port ${tcp_port} --password xxx)

query_decimal=$(cat <<-END
SELECT
    toTypeName(f_decimal32) as decimal32_ct,
    toTypeName(toDecimal32(0.0, 1)) as decimal32,
    toTypeName(f_decimal64) as decimal64_ct,
    toTypeName(toDecimal64(0.0, 2)) as decimal64,
    toTypeName(f_decimal128) as decimal128_ct,
    toTypeName(toDecimal128(0.0, 3)) as decimal128,
    toTypeName(f_decimal256) as decimal256_ct,
    toTypeName(toDecimal256(0.0, 4)) as decimal256
FROM t_testing_type_decimal
END
)
$(echo ${query_decimal} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/decimal.txt")

query_decimal_drop_table="DROP TABLE t_testing_type_decimal"
$(echo ${query_decimal_drop_table} | ${bin_client} --port ${tcp_port} --password xxx)

# 
query_string=$(cat <<-END
SELECT
    toTypeName('') as string
END
)
$(echo ${query_string} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/string.txt")

# 
query_fixedstring=$(cat <<-END
SELECT
    toTypeName(toFixedString('foo', 8)) as fixedstring
END
)
$(echo ${query_fixedstring} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/fixedstring.txt")

# 
query_uuid=$(cat <<-END
SELECT
    toTypeName(toUUID('61f0c404-5cb3-11e7-907b-a6006ad3dba0')) as uuid
END
)
$(echo ${query_uuid} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/uuid.txt")

# 
query_date=$(cat <<-END
SELECT
    toTypeName(toDate('2021-03-01')) as date
END
)
$(echo ${query_date} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/date.txt")

# 
query_datetime=$(cat <<-END
SELECT
    toTypeName(toDateTime('2021-03-01 01:02:03')) as datetime,
    toTypeName(toDateTime('2021-03-01 01:02:03', 'UTC')) as datetime_utc,
    toTypeName(toDateTime('2021-03-01 01:02:03', 'Asia/Shanghai')) as datetime_shanghai
END
)
$(echo ${query_datetime} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/datetime.txt")

# 
query_datetime64=$(cat <<-END
SELECT
    toTypeName(toDateTime64('2021-03-01 01:02:03', 0)) as datetime64,
    toTypeName(toDateTime64('2021-03-01 01:02:03', 3, 'UTC')) as datetime64_utc,
    toTypeName(toDateTime64('2021-03-01 01:02:03', 9, 'Asia/Shanghai')) as datetime64_shanghai
END
)
$(echo ${query_datetime64} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/datetime64.txt")

# 
query_enum=$(cat <<-END
SELECT
    toTypeName(CAST('a', 'Enum(\'a\'=-128, \'b\'=127)')) as enum8,
    toTypeName(CAST('a', 'Enum16(\'a\'=-32768, \'b\'=32767)')) as enum16,
    toTypeName(CAST('0', 'Enum(\'0\'=0, \'1\'=1)')) as enum8_2
END
)
$(echo ${query_enum} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/enum.txt")

# 
query_lowcardinality=$(cat <<-END
SELECT
    toTypeName(toLowCardinality('')) as lowcardinality_string,
    toTypeName(toLowCardinality(toFixedString('', 1))) as lowcardinality_fixedstring,
    toTypeName(toLowCardinality(toDate('2021-03-01'))) as lowcardinality_date,
    toTypeName(toLowCardinality(toDateTime('2021-03-01 01:02:03'))) as lowcardinality_datetime,
    toTypeName(toLowCardinality(toUInt8(0))) as lowcardinality_uint8,
    toTypeName(toLowCardinality(toUInt16(0))) as lowcardinality_uint16,
    toTypeName(toLowCardinality(toUInt32(0))) as lowcardinality_uint32,
    toTypeName(toLowCardinality(toUInt64(0))) as lowcardinality_uint64,
    toTypeName(toLowCardinality(toInt8(0))) as lowcardinality_int8,
    toTypeName(toLowCardinality(toInt16(0))) as lowcardinality_int16,
    toTypeName(toLowCardinality(toInt32(0))) as lowcardinality_int32,
    toTypeName(toLowCardinality(toInt64(0))) as lowcardinality_int64,
    toTypeName(toLowCardinality(toFloat32(0))) as lowcardinality_float32,
    toTypeName(toLowCardinality(toFloat64(0))) as lowcardinality_float64,
    toTypeName(toLowCardinality(toNullable(''))) as lowcardinality_nullable_string,
    toTypeName(toNullable(toLowCardinality(''))) as lowcardinality_nullable_string_2,
    toTypeName(toLowCardinality(toIPv4('127.0.0.1'))) as lowcardinality_ipv4,
    toTypeName(toLowCardinality(toIPv6('2a02:aa08:e000:3100::2'))) as lowcardinality_ipv6
END
)
$(echo ${query_lowcardinality} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/lowcardinality.txt")

# 
query_nullable=$(cat <<-END
SELECT
    toTypeName(toNullable(toUInt8(0))) as nullable_uint8,
    toTypeName(toNullable(toUInt256(0))) as nullable_uint256,
    toTypeName(toNullable(toInt8(0))) as nullable_int8,
    toTypeName(toNullable(toInt256(0))) as nullable_int256,
    toTypeName(toNullable(toFloat64(0))) as nullable_float64,
    toTypeName(toNullable(toDecimal256(0.0, 4))) as nullable_decimal256,
    toTypeName(toNullable('')) as nullable_string,
    toTypeName(toNullable(toFixedString('', 1))) as nullable_fixedstring,
    toTypeName(toNullable(toUUID('61f0c404-5cb3-11e7-907b-a6006ad3dba0'))) as nullable_uuid,
    toTypeName(toNullable(toDate('2021-03-01'))) as nullable_date,
    toTypeName(toNullable(toDateTime('2021-03-01 01:02:03'))) as nullable_datetime,
    toTypeName(toNullable(toDateTime64('2021-03-01 01:02:03', 0))) as nullable_datetime64,
    toTypeName(toNullable(CAST('a', 'Enum(\'a\'=-128, \'b\'=127)'))) as nullable_enum8,
    toTypeName(NULL) as nullable_nothing,
    toTypeName(toNullable(toIPv4('127.0.0.1'))) as nullable_ipv4,
    toTypeName(toNullable(toIPv6('2a02:aa08:e000:3100::2'))) as nullable_ipv6
END
)
$(echo ${query_nullable} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/nullable.txt")

# 
query_array=$(cat <<-END
SELECT
    toTypeName(array(toUInt8(0))) as array_uint8,
    toTypeName(array(toLowCardinality(toUInt8(0)))) as array_lowcardinality_uint8,
    toTypeName(array(toNullable(toUInt8(0)))) as array_nullable_uint8,
    toTypeName(array(array(toUInt8(0)))) as array_array_uint8,
    toTypeName(array(tuple(toUInt8(0), NULL))) as array_tuple_uint8_nullable_nothing
END
)
$(echo ${query_array} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/array.txt")

# 
query_tuple=$(cat <<-END
SELECT
    toTypeName(tuple('', toUInt8(0))) as tuple_string_uint8,
    toTypeName(tuple('', toLowCardinality(toUInt8(0)))) as tuple_string_lowcardinality_uint8,
    toTypeName(tuple('', toNullable(toUInt8(0)))) as tuple_string_nullable_uint8,
    toTypeName(tuple('', array(toUInt8(0)))) as tuple_string_array_uint8,
    toTypeName(tuple('', tuple(toUInt8(0), NULL))) as tuple_string_tuple_uint8_nullable_nothing
END
)
$(echo ${query_tuple} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/tuple.txt")

# 
query_ipv4=$(cat <<-END
SELECT
    toTypeName(toIPv4('127.0.0.1')) as ipv4
END
)
$(echo ${query_ipv4} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/ipv4.txt")

# 
query_ipv6=$(cat <<-END
SELECT
    toTypeName(toIPv6('2a02:aa08:e000:3100::2')) as ipv4
END
)
$(echo ${query_ipv6} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --port ${tcp_port} --password xxx > "${files_path}/ipv6.txt")

# 
query_map=$(cat <<-END
SELECT
    toTypeName(CAST((['1', '2'], ['a', 'b']), 'Map(String, String)')) as map_string_string,
    toTypeName(CAST((['1', '2'], ['a', 'b']), 'Map(FixedString(2), String)')) as map_fixedstring_string,
    toTypeName(CAST(([1, 2], ['a', 'b']), 'Map(UInt256, String)')) as map_uint256_string,
    toTypeName(CAST(([1, 2], ['a', 'b']), 'Map(Int256, String)')) as map_int256_string,
    toTypeName(CAST(([1, 2], ['a', 'b']), 'Map(Float64, String)')) as map_float64_string,
    toTypeName(CAST(([1, 2], ['a', 'b']), 'Map(Decimal(9, 9), String)')) as map_decimal_string,
    toTypeName(CAST((['1', '2'], [array('a'), array('b')]), 'Map(String, Array(String))')) as map_string_array_string
END
)
$(echo ${query_map} FORMAT JSONCompactEachRowWithNamesAndTypes | ${bin_client} --allow_experimental_map_type 1 --port ${tcp_port} --password xxx > "${files_path}/map.txt")


sleep 1
