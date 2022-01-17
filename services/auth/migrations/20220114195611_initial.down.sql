-- user
DROP TABLE IF EXISTS user;
DROP TABLE IF EXISTS user_update_history;
DROP TABLE IF EXISTS user;
DROP INDEX idx_user_uuid;
DROP INDEX idx_user_update_uuid;
DROP TRIGGER IF EXISTS on_user_email_before_insert;
DROP TRIGGER IF EXISTS on_user_after_update;
-- authentication
DROP TABLE IF EXISTS authentication;
DROP TABLE IF EXISTS authentication_update_history;
DROP TRIGGER IF EXISTS on_authentication_update;
DROP INDEX IF EXISTS idx_auth_user_uuid;
DROP INDEX IF EXISTS idx_auth_update_uuid;
-- -- views
-- DROP VIEW IF EXISTS v_user;
-- DROP VIEW IF EXISTS v_authenticated_users;
-- DROP VIEW IF EXISTS v_auth_history;
