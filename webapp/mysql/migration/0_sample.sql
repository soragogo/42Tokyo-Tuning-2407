-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
CREATE INDEX user_unsernae ON users (username);

CREATE INDEX dispatchers_user_id ON dispatchers (user_id);

CREATE INDEX sessions_session_token ON sessions (session_token);

