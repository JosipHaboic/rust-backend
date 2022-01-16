-- .log db.log
-- .headers ON
-- .mode column
-- TABLES
--
CREATE TABLE IF NOT EXISTS user (
	id TEXT PRIMARY KEY NOT NULL,
	username TEXT NOT NULL UNIQUE,
	password TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
	CHECK(length(password) > 0)
);
--
CREATE TABLE IF NOT EXISTS user_update_history (
	id INTEGER PRIMARY KEY,
	usr_id TEXT NOT NULL,
	username TEXT NULL,
	password TEXT NULL,
	email TEXT NULL,
	updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);
-- INDEXES
--
CREATE UNIQUE INDEX IF NOT EXISTS idx_user_id ON user(id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_usrer_update_id ON user_update_history(id);
-- TRIGGERS
--
CREATE TRIGGER IF NOT EXISTS on_user_email_before_insert BEFORE
INSERT ON user BEGIN
SELECT CASE
		WHEN NEW.email NOT LIKE '%_@_%.__%' THEN RAISE (ABORT, 'Invalid email address')
	END;
END;
--
CREATE TRIGGER IF NOT EXISTS on_user_after_update
AFTER
UPDATE ON user
	WHEN old.username <> new.username
	OR old.password <> new.password
	OR old.email <> new.email BEGIN
INSERT INTO user_update_history (usr_id, username, password, email, updated_at)
VALUES (
		old.id,
		old.username,
		old.password,
		old.email,
		datetime('now', 'localtime')
	);
END;
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
CREATE TABLE IF NOT EXISTS authentication_update_history (
	id INTEGER PRIMARY KEY,
	usr_id TEXT NOT NULL,
	passed TEXT NULL,
	updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
	FOREIGN KEY(usr_id) REFERENCES authentication(usr_id)
	ON UPDATE CASCADE
	ON DELETE CASCADE
);
-- INDEXES
--
CREATE UNIQUE INDEX IF NOT EXISTS idx_auth_usr_id ON authentication(usr_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_auth_update_id ON authentication_update_history(usr_id);
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
