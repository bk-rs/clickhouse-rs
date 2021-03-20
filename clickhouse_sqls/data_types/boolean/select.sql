SELECT
    true_val, 
    toTypeName(true_val) AS true_ty,
    false_val,
    toTypeName(false_val) AS false_ty
FROM t_boolean
