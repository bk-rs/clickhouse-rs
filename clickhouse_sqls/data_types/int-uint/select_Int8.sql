SELECT
    toInt8(-128) AS min_val,
    toTypeName(min_val) AS min_ty,
    toInt8(127) AS max_val,
    toTypeName(max_val) AS max_ty
