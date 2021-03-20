SELECT
    x AS x_val,
    toTypeName(x_val) AS x_ty,
    ifNull(y, -1) AS y_val,
    toTypeName(y_val) AS y_ty
FROM t_null
