SELECT
    toDateTime64('2021-03-01 01:02:03.123456789', 3, 'UTC') AS val,
    toTypeName(val) AS ty
