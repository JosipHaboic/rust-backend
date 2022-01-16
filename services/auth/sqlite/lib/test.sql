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
UPDATE authentication
SET passed = 'TRUE'
WHERE usr_id = '3';
