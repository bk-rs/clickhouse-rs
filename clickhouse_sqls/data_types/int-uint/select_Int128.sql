SELECT
    toInt128('-170141183460469231731687303715884105728') AS min_val,
    toTypeName(min_val) AS min_ty,
    toInt128('170141183460469231731687303715884105727') AS max_val,
    toTypeName(max_val) AS max_ty
