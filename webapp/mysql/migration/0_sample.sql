-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
CREATE INDEX user_unsernae ON users (username);

CREATE INDEX dispatchers_user_id ON dispatchers (user_id);

CREATE INDEX sessions_session_token ON sessions (session_token);

CREATE INDEX idx_tow_truck_id_timestamp ON locations (tow_truck_id, timestamp);

UPDATE tabluserse_name SET password = '5f4dcc3b5aa765d61d8327deb882cf99';
