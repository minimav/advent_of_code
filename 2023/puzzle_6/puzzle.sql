DROP TABLE IF EXISTS puzzle_6;

CREATE TABLE puzzle_6 (
   id INTEGER PRIMARY KEY,
   time INTEGER NOT NULL,
   distance INTEGER NOT NULL
);

-- Example part 1
--INSERT INTO puzzle_6 (id, time, distance) VALUES (1, 7, 9), (2, 15, 40), (3, 30, 200);
-- Example part 2
--INSERT INTO puzzle_6(id, time, distance) VALUES (4, 71530, 940200);
-- Input part 1
--INSERT INTO puzzle_6 (id, time, distance) VALUES (1, 49, 298), (2, 78, 1185), (3, 79, 1066), (4, 80, 1181);
-- Input part 2
INSERT INTO puzzle_6(id, time, distance) VALUES (5, 49787980, 298118510661181);

SELECT
    --No product in SQL boo...
    ROUND(exp(sum(log(high - low + 1))), 0) AS answer
FROM (
    SELECT
        id,
        CASE WHEN raw_low = ROUND(raw_low, 0) THEN raw_low + 1 ELSE CEIL(raw_low) END AS low,
        CASE WHEN raw_high = ROUND(raw_high, 0) THEN raw_high - 1 ELSE FLOOR(raw_high) END AS high
    FROM (
        SELECT
            id,
            (time - SQRT(time * time - 4 * distance)) / 2 AS raw_low,
            (time + SQRT(time * time - 4 * distance)) / 2 AS raw_high
        FROM puzzle_6
    )
);