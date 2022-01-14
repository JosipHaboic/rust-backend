--
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
--
CREATE TABLE IF NOT EXISTS user_update_history (
	id INTEGER PRIMARY KEY,
	usr_id TEXT NOT NULL,
	username TEXT NULL,
	password TEXT NULL,
	email TEXT NULL,
	updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);
--
-- INDEXES
--
CREATE UNIQUE INDEX IF NOT EXISTS idx_user_id ON user(id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_usrer_update_id ON user_update_history(id);
--
-- TRIGGERS
--
CREATE TRIGGER IF NOT EXISTS on_user_email_before_insert BEFORE
INSERT ON user BEGIN
SELECT CASE
		WHEN NEW.email NOT LIKE '%_@_%.__%' THEN RAISE (ABORT, 'Invalid email address')
	END;
END;
--
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
--
-- VIEWS
--
CREATE VIEW v_user AS
SELECT user.id,
	user.username,
	user.created_at,
	authorization.type as authorization,
	authentication.passed as authenticated
FROM user
	INNER JOIN authorization ON authorization.usr_id = user.id
	INNER JOIN authentication ON authentication.usr_id = user.id;
--
--
CREATE VIEW v_authenticated_users AS
SELECT 
	user.id,
	user.username,
	user.created_at
	FROM user
	INNER JOIN authentication ON authentication.usr_id = user.id;
--
--
CREATE VIEW v_auth_history AS
SELECT 
	authentication_update_history.usr_id,
	user.username AS username,
	authentication_update_history.passed,
	authentication_update_history.updated_at
FROM authentication_update_history
	JOIN user ON user.id = authentication_update_history.usr_id;
--
-- TEST DATA
--
INSERT INTO user (id, username, password, email)
VALUES ('1', 'John', '12345678', 'john@gmail.com');
--
INSERT INTO user (id, username, password, email)
VALUES (
		'2',
		'Jane',
		'ds9ds9dss89d8s',
		'jane@gmail.com'
	);
--
INSERT INTO user (id, username, password, email)
VALUES (
		'3',
		'Ana',
		'erze8e8re87re8',
		'ana@smail.com'
	);
--
INSERT INTO user (id, username, password, email)
VALUES (
		'4',
		'Mike',
		'lckls8d9s8_9ds8_98',
		'mike@gmail.com'
	);
--
INSERT INTO authentication (usr_id, passed)
VALUES ('1', 'FALSE');
--
INSERT INTO authentication (usr_id, passed)
VALUES ('2', 'FALSE');
--
INSERT INTO authentication (usr_id, passed)
VALUES ('3', 'TRUE');
--
INSERT INTO authentication (usr_id, passed)
VALUES ('4', 'FALSE');
--
INSERT INTO authorization (usr_id, type)
VALUES ('1', 'Visitor');
--
INSERT INTO authorization (usr_id, type)
VALUES ('2', 'Visitor');
--
INSERT INTO authorization (usr_id, type)
VALUES ('3', 'Admin');
--
INSERT INTO authorization (usr_id, type)
VALUES ('4', 'Visitor');
--

UPDATE authentication
SET passed = 'TRUE'
WHERE usr_id = '3';

--
--

UPDATE authorization
SET type = 'Registered'
WHERE usr_id = '3';

--
--

