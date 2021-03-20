SELECT
    CAST('a', 'Enum(\'a\'=-128, \'b\'=127)') AS val,
    toTypeName(val) AS ty
