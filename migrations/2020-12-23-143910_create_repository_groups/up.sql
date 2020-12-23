CREATE TABLE repository_groups (
    uuid BINARY(16) PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    name VARCHAR(255) NOT NULL
);
