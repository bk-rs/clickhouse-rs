SELECT
    CAST(CAST('a', 'Enum(\'a\'=1, \'b\'=2)'), 'Int8') AS a_val,
    toTypeName(a_val) AS a_ty,
    CAST(CAST('b', 'Enum(\'a\'=1, \'b\'=2)'), 'Int16') AS b_val,
    toTypeName(b_val) AS b_ty
