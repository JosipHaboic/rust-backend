--
-- TABLES
--
CREATE TABLE IF NOT EXISTS authentication (
	id INTEGER PRIMARY KEY,
	usr_id TEXT NOT NULL UNIQUE,
	passed TEXT NOT NULL DEFAULT "FALSE",
	FOREIGN KEY(usr_id) REFERENCES user(id)
	ON UPDATE CASCADE
	ON DELETE CASCADE
);
--
--
CREATE TABLE IF NOT EXISTS authentication_update_history (
	id INTEGER PRIMARY KEY,
	usr_id TEXT NOT NULL,
	passed TEXT NULL,
	updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
	FOREIGN KEY(usr_id) REFERENCES authentication(usr_id)
	ON UPDATE CASCADE
	ON DELETE CASCADE
);
--
-- TRIGGERS
--
CREATE TRIGGER IF NOT EXISTS on_authentication_update
AFTER
UPDATE ON authentication
	WHEN old.passed <> new.passed BEGIN
UPDATE authentication_update_history 
SET 
	passed = new.passed,
	updated_at = datetime('now', 'localtime')
WHERE usr_id = new.usr_id;
END;