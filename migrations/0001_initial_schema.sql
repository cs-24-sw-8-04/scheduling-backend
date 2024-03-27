CREATE TABLE Accounts(
  id     INTEGER PRIMARY KEY NOT NULL,
  username      VARCHAR(255) NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  UNIQUE(username)
);

CREATE TABLE AuthTokens(
  id VARCHAR(64) PRIMARY KEY NOT NULL,
  account_id         INTEGER NOT NULL
    REFERENCES Accounts(id) ON DELETE CASCADE
);

CREATE TABLE Tasks(
  id  INTEGER PRIMARY KEY NOT NULL,
  timespan_start DATETIME NOT NULL,
  timespan_end   DATETIME NOT NULL,
  duration       INTEGER  NOT NULL,
  effect         REAL     NOT NULL,
  account_id     INTEGER  NOT NULL
    REFERENCES Accounts(id) ON DELETE CASCADE
);
