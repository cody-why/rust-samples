CREATE DATABASE IF NOT EXISTS casbin default charset utf8mb4 COLLATE utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS casbin.casbin_rule (
    id BIGINT unsigned NOT NULL AUTO_INCREMENT,
    ptype VARCHAR(12) NOT NULL,
    v0 VARCHAR(128) NOT NULL,
    v1 VARCHAR(128) NOT NULL,
    v2 VARCHAR(128) NOT NULL,
    v3 VARCHAR(128) NOT NULL,
    v4 VARCHAR(128) NOT NULL,
    v5 VARCHAR(128) NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT unique_key_sqlx_adapter UNIQUE(ptype, v0, v1, v2, v3, v4, v5),
    KEY idx_name (v0) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
 
