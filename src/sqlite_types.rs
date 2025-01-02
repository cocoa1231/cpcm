use serde::{Serialize, Deserialize};
use rusqlite::Row;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum SqlWhere {
    Like(String),
    GreaterThan(i32),
    LessThan(i32),
    EqualTo(i32),
    GreaterEqual(i32),
    LessEqual(i32)
}

#[allow(dead_code)]
pub struct SqlWhereFilter {
    colname: String,
    filter: SqlWhere,
}
impl SqlWhereFilter {
    pub fn new() -> Self {
        todo!()
    }
}


// Represents a row in the table of domains. This is also the response recieved
// from whmapi1's get_domain_info. Specifically it's data.domains[]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRow {
    pub docroot: Option<String>,
    pub domain: Option<String>,
    pub domain_type: Option<String>,
    pub ipv4: Option<String>,
    pub ipv4_ssl:  Option<String>,
    pub ipv6: Option<String>,
    pub ipv6_is_dedicated: Option<i32>,
    pub modsecurity_enabled: Option<i32>,
    pub parent_domain: Option<String>,
    pub php_version: Option<String>,
    pub port: Option<String>,
    pub port_ssl: Option<String>,
    pub user: Option<String>,
    pub user_owner: Option<String>
}

impl DomainRow {

    pub fn header_str() -> Vec<String> {
        vec![
            "docroot",
            "domain",
            "domain_type",
            "ipv4",
            "ipv4_ssl",
            "ipv6",
            "modsecurity_enabled",
            "parent_domain",
            "php_version",
            "port",
            "port_ssl",
            "user",
            "user_owner"
        ].into_iter()
            .map(|x| x.to_string())
            .collect()
    }

    pub fn from_row(r: &Row) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            docroot: Some(r.get::<_, String>("docroot")?),
            domain: Some(r.get::<_, String>("domain")?),
            domain_type: Some(r.get::<_, String>("domain_type")?),
            ipv4: Some(r.get::<_, String>("ipv4")?),
            ipv4_ssl: Some(r.get::<_,  String>("ipv4_ssl")?),
            ipv6: Some(r.get::<_, String>("ipv6")?),
            ipv6_is_dedicated: Some(r.get::<_, i32>("ipv6_is_dedicated")?),
            modsecurity_enabled: Some(r.get::<_, i32>("modsecurity_enabled")?),
            parent_domain: Some(r.get::<_, String>("parent_domain")?),
            php_version: Some(r.get::<_, String>("php_version")?),
            port: Some(r.get::<_, String>("port")?),
            port_ssl: Some(r.get::<_, String>("port_ssl")?),
            user: Some(r.get::<_, String>("user")?),
            user_owner: Some(r.get::<_, String>("user_owner")?)
        })
    }

    // bless vim macros
    pub fn as_vec(self) -> Vec<String> {
        let ipv6 = match self.ipv6_is_dedicated {
            Some(i) => i.to_string(),
            None => "NULL".to_string()
        };
        let modsecurity_enabled = match self.modsecurity_enabled {
            Some(m) => m.to_string(),
            None => "NULL".to_string()
        };
        let v = vec![
            self.docroot,
            self.domain,
            self.domain_type,
            self.ipv4,
            self.ipv4_ssl,
            Some(ipv6),
            Some(modsecurity_enabled),
            self.parent_domain,
            self.php_version,
            self.port,
            self.port_ssl,
            self.user,
            self.user_owner
        ];

        v.into_iter()
            .map(|x| match x {
                Some(s) =>  s,
                None => String::from("NULL")
            })
            .collect()
    }

    fn nullable(s: Option<String>) -> String {
        match s {
            Some(s) => s.to_string(),
            None => "NULL".to_string()
        }
    }

    fn nullable_i32(s: Option<i32>) -> i32 {
        match s {
            Some(s) => s,
            None => 0
        }
    }

    pub fn safe_unwrap(self) -> Result<Self, Box<dyn Error>> {
        let docroot = self.docroot.ok_or("Docroot not provided!")?;
        let domain = self.domain.ok_or("Domain not provided!")?;
        let domain_type = self.domain_type.ok_or("Unable to determine domain type!")?;
        let ipv4 = self.ipv4.ok_or("IPv4 address not provided!")?;
        let ipv4_ssl = DomainRow::nullable(self.ipv4_ssl);
        let ipv6 = DomainRow::nullable(self.ipv6);
        let ipv6_is_dedicated = DomainRow::nullable_i32(self.ipv6_is_dedicated);
        let modsecurity_enabled = DomainRow::nullable_i32(self.modsecurity_enabled);
        let parent_domain = DomainRow::nullable(self.parent_domain);
        let php_version = DomainRow::nullable(self.php_version);
        let port = DomainRow::nullable(self.port);
        let port_ssl = DomainRow::nullable(self.port_ssl);
        let user = self.user.ok_or("User not provided!")?;
        let user_owner = DomainRow::nullable(self.user_owner);

        Ok(Self {
            docroot: Some(docroot),
            domain: Some(domain),
            domain_type: Some(domain_type),
            ipv4: Some(ipv4),
            ipv4_ssl: Some(ipv4_ssl),
            ipv6: Some(ipv6),
            ipv6_is_dedicated: Some(ipv6_is_dedicated),
            modsecurity_enabled: Some(modsecurity_enabled),
            parent_domain: Some(parent_domain),
            php_version: Some(php_version),
            port: Some(port),
            port_ssl: Some(port_ssl),
            user: Some(user),
            user_owner: Some(user_owner)
        })
    }
}
