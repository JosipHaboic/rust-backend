--
-- TABLES
--
CREATE TABLE IF NOT EXISTS authorization (
	id INTEGER PRIMARY KEY,
	usr_id TEXT NOT NULL UNIQUE,
	type TEXT NOT NULL DEFAULT "NONE",
	created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
	FOREIGN KEY(usr_id) REFERENCES user(id)
	ON UPDATE CASCADE
	ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS authorization_update_history (
	id INTEGER PRIMARY KEY,
	usr_id TEXT NOT NULL,
	type TEXT NOT NULL,
	updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);
--
-- TRIGGERS
--
CREATE TRIGGER IF NOT EXISTS on_authorization_update
AFTER
UPDATE ON authorization
	WHEN old.type <> new.type
	OR old.usr_id <> new.usr_id BEGIN
INSERT INTO authorization_update_history (usr_id, type)
VALUES (new.usr_id, new.type);
END;
