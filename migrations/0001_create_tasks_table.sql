CREATE TABLE Tasks(
  id INTEGER PRIMARY KEY NOT NULL,
  timespan_start DATETIME NOT NULL,
  timespan_end   DATETIME NOT NULL,
  duration       INTEGER NOT NULL,
  effect         REAL    NOT NULL
);

