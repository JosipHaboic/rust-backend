BEGIN TRANSACTION;
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
COMMIT;
--
BEGIN TRANSACTION;
UPDATE authentication
SET passed = 'TRUE'
WHERE usr_id = '3';
COMMIT;
--
--
BEGIN TRANSACTION;
UPDATE authorization
SET type = 'Registered'
WHERE usr_id = '3';
COMMIT;
--
--
BEGIN TRANSACTION;
SELECT *
FROM user;
--
SELECT *
FROM user_update_history;
--
SELECT *
FROM authentication;
--
SELECT *
FROM authentication_update_history;
--
SELECT *
FROM authorization;
--
END;
--
BEGIN TRANSACTION;
SELECT *
FROM v_user;
SELECT *
FROM v_authenticated_users;
SELECT *
FROM v_auth_history;
END;