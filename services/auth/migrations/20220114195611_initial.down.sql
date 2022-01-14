-- user
DROP TABLE IF EXISTS user;
DROP TABLE IF EXISTS user_update_history;
DROP TABLE IF EXISTS user;
DROP INDEX idx_user_id;
DROP INDEX idx_usrer_update_id;
DROP TRIGGER IF EXISTS on_user_email_before_insert;
DROP TRIGGER IF EXISTS on_user_after_update;
-- authentication
DROP TABLE IF EXISTS authentication;
DROP TABLE IF EXISTS authentication_update_history;
DROP TRIGGER IF EXISTS on_authentication_update;
-- authorization
DROP TABLE IF EXISTS authorization;
DROP TABLE IF EXISTS authorization_update_history;
DROP TRIGGER IF EXISTS on_authorization_update;
---
-- views
---
DROP VIEW IF EXISTS v_user;
DROP VIEW IF EXISTS v_authenticated_users;
DROP VIEW IF EXISTS v_auth_history;
