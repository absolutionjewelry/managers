-- Add up migration script here
CREATE TABLE IF NOT EXISTS authentications (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    token TEXT NOT NULL,
    expires_at VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    CONSTRAINT foreign_key_user_id FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_authentications_user_id ON authentications(user_id);
CREATE INDEX IF NOT EXISTS idx_authentications_token ON authentications(token);
CREATE INDEX IF NOT EXISTS idx_authentications_expires_at ON authentications(expires_at);
CREATE INDEX IF NOT EXISTS idx_authentications_created_at ON authentications(created_at);
CREATE INDEX IF NOT EXISTS idx_authentications_archived_at ON authentications(archived_at);
