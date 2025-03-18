-- Add up migration script here
CREATE TABLE authentications (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    token TEXT NOT NULL,
    expires_at VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT foreign_key_user_id FOREIGN KEY (user_id) REFERENCES users(id)
);
