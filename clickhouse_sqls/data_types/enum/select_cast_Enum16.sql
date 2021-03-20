SELECT
    CAST(CAST('a', 'Enum16(\'a\'=1, \'b\'=2)'), 'Int16') AS val,
    toTypeName(val) AS ty
