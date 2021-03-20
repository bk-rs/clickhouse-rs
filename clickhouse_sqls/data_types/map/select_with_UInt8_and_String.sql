SELECT
    CAST(([1, 2, 3], ['Ready', 'Steady', 'Go']), 'Map(UInt8, String)') AS val,
    toTypeName(val) AS ty
