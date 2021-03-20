SELECT
    CAST('a', 'Enum16(\'a\'=-32768, \'b\'=32767)') AS val,
    toTypeName(val) AS ty
