SELECT
    toInt16(-32768) AS min_val,
    toTypeName(min_val) AS min_ty,
    toInt16(32767) AS max_val,
    toTypeName(max_val) AS max_ty
