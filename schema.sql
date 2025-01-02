CREATE TABLE IF NOT EXISTS servers(
  `name`     TEXT,
  `ip`       TEXT,
  `user`     TEXT,
  `apikey`   TEXT,
  `hostname` TEXT,
  `group`    TEXT,
  PRIMARY KEY(`name`,`ip`)
);

CREATE TABLE IF NOT EXISTS domains(
  `lastupdated` INTEGER,
  `server_name` TEXT,
  `docroot` TEXT,
  `domain` TEXT,
  `domain_type` TEXT,
  `ipv4` TEXT,
  `ipv4_ssl` TEXT,
  `ipv6` TEXT,
  `ipv6_is_dedicated` TEXT,
  `modsecurity_enabled` TEXT,
  `parent_domain` TEXT,
  `php_version` TEXT,
  `port` TEXT,
  `port_ssl` TEXT,
  `user` TEXT,
  `user_owner` TEXT,
  PRIMARY KEY(`server_name`, `domain`),
  FOREIGN KEY(`server_name`) REFERENCES servers(`name`)
);

-- Search by index
CREATE INDEX domain_idx ON `domains`(`domain`);
