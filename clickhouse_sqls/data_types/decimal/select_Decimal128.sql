SELECT
    toDecimal128(toString(-1.111), 5) AS val,
    toTypeName(val) AS ty
