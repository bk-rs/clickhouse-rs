SELECT
    toInt64(-9223372036854775808) AS min_val,
    toTypeName(min_val) AS min_ty,
    toInt64(9223372036854775807) AS max_val,
    toTypeName(max_val) AS max_ty
