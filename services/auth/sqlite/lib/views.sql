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
--