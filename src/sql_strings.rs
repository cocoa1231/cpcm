
use crate::config::Config;

#[allow(non_snake_case)]
pub fn SQLSCHEMA(config: &Config) -> String {
    return format!(r#"

CREATE TABLE IF NOT EXISTS {}(
  `name`     TEXT,
  `ip`       TEXT,
  `user`     TEXT,
  `apikey`   TEXT,
  `hostname` TEXT,
  `group`    TEXT,
  PRIMARY KEY(`name`,`ip`)
);

-- Statement
CREATE TABLE IF NOT EXISTS {}(
  `lastupdated` INTEGER,
  `server_name` TEXT,
  `server_ip` TEXT,
  `docroot` TEXT,
  `domain` TEXT,
  `domain_type` TEXT,
  `ipv4` TEXT,
  `ipv4_ssl` TEXT,
  `ipv6` TEXT,
  `ipv6_is_dedicated` INTEGER,
  `modsecurity_enabled` INTEGER,
  `parent_domain` TEXT,
  `php_version` TEXT,
  `port` TEXT,
  `port_ssl` TEXT,
  `user` TEXT,
  `user_owner` TEXT,
  PRIMARY KEY(`server_name`, `domain`),
  FOREIGN KEY(`server_name`, `server_ip`) REFERENCES {}(`name`, `ip`)
);

-- Statement
CREATE INDEX domain_idx ON `{}`(`domain`);

"#, config.tabname_server(), config.tabname_domain(), config.tabname_server(), config.tabname_domain())
}

#[allow(non_snake_case)]
pub fn SERVERADD_UPSERT(config: &Config) -> String {
    return format!(r#"
INSERT INTO {}(`name`, `ip`, `user`, `apikey`, `hostname`, `group`)
VALUES (:name, :ip, :user, :apikey, :hostname, :group)
ON CONFLICT (name, ip) DO UPDATE SET
    `name`=excluded.`name`,
    `ip`=excluded.`ip`,
    `user`=excluded.`user`,
    `apikey`=excluded.`apikey`,
    `hostname`=excluded.`hostname`,
    `group`=excluded.`group`;"#, config.tabname_server())
}

#[allow(non_snake_case)]
pub fn DOMAINSYNC_UPSERT(config: &Config) -> String {
    return format!(r#"
INSERT INTO `{}`(docroot, domain, domain_type, ipv4, ipv4_ssl, ipv6, ipv6_is_dedicated, modsecurity_enabled, parent_domain, php_version, port, port_ssl, user, user_owner, server_name, server_ip, lastupdated)
VALUES(:docroot, :domain, :domain_type, :ipv4, :ipv4_ssl, :ipv6, :ipv6_is_dedicated, :modsecurity_enabled, :parent_domain, :php_version, :port, :port_ssl, :user, :user_owner, :server_name, :server_ip, :lastupdate)
    ON CONFLICT (server_name, domain) DO UPDATE SET
        server_name=excluded.server_name,
        server_ip=excluded.server_ip,
        docroot=excluded.docroot,
        domain=excluded.domain,
        domain_type=excluded.domain_type,
        ipv4=excluded.ipv4,
        ipv4_ssl=excluded.ipv4_ssl,
        ipv6=excluded.ipv6,
        ipv6_is_dedicated=excluded.ipv6_is_dedicated,
        modsecurity_enabled=excluded.modsecurity_enabled,
        parent_domain=excluded.parent_domain,
        php_version=excluded.php_version,
        port=excluded.port,
        port_ssl=excluded.port_ssl,
        user=excluded.user,
        user_owner=excluded.user_owner,
        lastupdated=excluded.lastupdated
    WHERE lastupdated>excluded.lastupdated;"#, config.tabname_domain())
}
 
