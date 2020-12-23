CREATE TABLE repositories (
    uuid BINARY(16) PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    path VARCHAR(255) NOT NULL,
    group_uuid BINARY(16)
);
