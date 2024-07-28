-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
CREATE INDEX user_unsernae ON users (username);

CREATE INDEX dispatchers_user_id ON dispatchers (user_id);

CREATE INDEX sessions_session_token ON sessions (session_token);

CREATE INDEX idx_tow_truck_id_timestamp ON locations (tow_truck_id, timestamp);

UPDATE users SET password = '5f4dcc3b5aa765d61d8327deb882cf99';

CREATE INDEX idx_nodes_area_id ON nodes(area_id);

CREATE INDEX idx_edges_node_a_id ON edges(node_a_id);

-- CREATE INDEX idx_nodes_id ON nodes(id);

-- CREATE INDEX idx_orders_status ON orders(status);

-- CREATE INDEX idx_tow_trucks_status ON tow_trucks(status);
-- CREATE INDEX idx_tow_trucks_area_id ON tow_trucks(area_id);
-- CREATE INDEX idx_locations_tow_truck_id ON locations(tow_truck_id);
-- CREATE INDEX idx_locations_timestamp ON locations(timestamp);
