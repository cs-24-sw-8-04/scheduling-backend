CREATE TABLE Tasks(
  id INTEGER PRIMARY KEY NOT NULL,
  timeslot_start INTEGER NOT NULL,
  timeslot_end   INTEGER NOT NULL,
  duration       INTEGER NOT NULL,
  effect         REAL    NOT NULL
);

