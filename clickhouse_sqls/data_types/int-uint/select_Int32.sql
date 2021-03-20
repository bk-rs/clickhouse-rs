SELECT
    toInt32(-2147483648) AS min_val,
    toTypeName(min_val) AS min_ty,
    toInt32(2147483647) AS max_val,
    toTypeName(max_val) AS max_ty
