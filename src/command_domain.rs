use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;


use clap::Args;
use reqwest::{header, ClientBuilder};
use rusqlite::{Connection, params};
use serde_json::Value;
use http::Method;
use url::Url;

use crate::global_paths::GlobalPaths;
use crate::config::Config;
use crate::sqlite_types::DomainRow;
use crate::sql_strings::DOMAINSYNC_UPSERT;

#[derive(Debug, Args)]
pub struct DomainArgs {
    #[arg(long)]
    sync: bool,

    #[arg(long, short)]
    name: Option<String>,
}



async fn sync_domain_db(paths: &GlobalPaths, config: &Config) -> Result<(), Box<dyn Error>> {
    let lastupdate = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().to_string();
    let db = Connection::open(paths.dbfile())?;

    // I'm fine panicing if the domain table name doesn't exist
    // because otherwise something has gone horribly wrong.

    let remove_sql = format!("DELETE FROM {} WHERE lastupdated < {}", config.tabname_domain(), &lastupdate);
    let mut upsert_stmt = db.prepare(&DOMAINSYNC_UPSERT(config))?;
    let mut remove_stmt =db.prepare(&remove_sql)?;
    let mut statement = db.prepare("SELECT name,ip,hostname,user,apikey FROM servers")?;
    let mut servers = statement.query(params![])?;
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    // Each row is one server
    while let Some(row) = servers.next()? {
        let server_name: String = row.get::<_, String>("name")?.clone();
        let server_ip: String = row.get::<_, String>("ip")?.clone();
        let user: String = row.get::<_, String>("user")?.clone();
        // query endpoint
        let whmurl = Url::parse(
            &format!("https://{}:2087/json-api/get_domain_info?api.version=1", row.get::<_, String>("ip")?).to_string()
        )?;
        let authkey = row.get::<_, String>("apikey")?;
        let req = client.request(Method::GET, whmurl)
            .header(
                header::HeaderName::from_static("authorization"),
                header::HeaderValue::from_str(&format!("whm {}:{}", user, authkey))?
            )
            .build()?;

        log::debug!("Sending request to {} via IP {}", server_name, server_ip);
        log::debug!("{}", format!("Request details {:?}", req));
        let resp = match client.execute(req).await {
            Ok(r) => {
                log::debug!("Got response {:?}", r);
                r.json::<Value>().await?
            }
            Err(e) => {
                log::error!("{:?}", e);
                std::process::exit(2);
            }
        };

        log::debug!("Response data\n{:?}", resp);
        let domains = match resp["data"]["domains"].as_array() {
            Some(x) => {
                let domains = x
                    .into_iter()
                    .inspect(|&x| log::debug!("{:?}", x))
                    .map(|x| {
                        let v = serde_json::from_value::<DomainRow>(x.clone());
                        if v.is_err() {
                            log::debug!("Unable to convert row! {}", v.err().unwrap());
                            return None
                        } else {
                            return v.ok()
                        }

                    })
                    .collect::<Vec<Option<DomainRow>>>();
                domains
            }
            None => {
                let domains: Vec<Option<DomainRow>> = Vec::new();
                domains
            }
            };
        //let domains = serde_json::from_value::<Vec<DomainRow>>(resp["data"]["domains"].clone())?;

        // Perform upsert. Good lord why is this so annoying just implement
        // rusqlite::Params for std::vec::Vec.
        for domain_row in domains {
            if let Some(domain_row) = domain_row {
                let domain_row_result = domain_row.safe_unwrap();
                if let Err(e) = domain_row_result {
                    log::error!("Unable to unwrap row! {:?}", e);
                    continue;
                }
                let safe_domain_row = domain_row_result.unwrap();

                log::debug!("Inserting row {:?}", &safe_domain_row);
                let u = upsert_stmt.execute(rusqlite::named_params! {
                    ":docroot": safe_domain_row.docroot.unwrap(),
                    ":domain": safe_domain_row.domain.unwrap(),
                    ":domain_type": safe_domain_row.domain_type.unwrap(),
                    ":ipv4": safe_domain_row.ipv4.unwrap(),
                    ":ipv4_ssl": safe_domain_row.ipv4_ssl.unwrap(),
                    ":ipv6": safe_domain_row.ipv6.unwrap_or("NULL".to_string()),
                    ":ipv6_is_dedicated": safe_domain_row.ipv6_is_dedicated.unwrap_or(0),
                    ":modsecurity_enabled": safe_domain_row.modsecurity_enabled.unwrap(),
                    ":parent_domain": safe_domain_row.parent_domain.unwrap(),
                    ":php_version": safe_domain_row.php_version.unwrap(),
                    ":port": safe_domain_row.port.unwrap(),
                    ":port_ssl": safe_domain_row.port_ssl.unwrap(),
                    ":user": safe_domain_row.user.unwrap(),
                    ":user_owner": safe_domain_row.user_owner.unwrap(),
                    ":server_name": server_name,
                    ":server_ip": server_ip,
                    ":lastupdate": lastupdate
                })?;

                log::debug!("Inserted row with status code {u}");
            } else {
                log::debug!("Unable to parse row(s)!");
            }
        }


    };

    // Finally clean up outdated rows
    remove_stmt.execute(params![])?;

    Ok(())
}


fn find_and_print_domains(name: String, paths: &GlobalPaths, config: &Config)
-> Result<(), Box<dyn Error>> {

    log::debug!("Filtering domains containing  {}", &name);
    let db = Connection::open(paths.dbfile())?;
    
    log::debug!("Connected to database");
    let filter_sql = format!("SELECT * FROM {} WHERE domain LIKE '%{}%';", config.tabname_domain(), name.trim());
    let mut filter_stmt = db.prepare(&filter_sql)?;

    let mut results = filter_stmt.query(params![])?;
    let mut builder = tabled::builder::Builder::new();

    builder.push_record(DomainRow::header_str());
    log::debug!("Retrieved required rows. Printing rows.");
    while let Ok(rowop) = results.next() {
        if let Some(row) = rowop {
            log::debug!("Found row {:?}", row);
            builder.push_record(DomainRow::from_row(row)?.as_vec());
        } else {
            break
        }
    }

    let mut table = builder.build();
    table.with(tabled::settings::Style::rounded());

    println!("{}", table.to_string());
    Ok(())
}

pub async fn run_domain(args: DomainArgs, paths: &GlobalPaths, config: &Config) 
    -> Result<(), Box<dyn Error>> { 
    dbg!(&args, &paths);

    if args.sync {
        log::info!("Syncing domains");
        let r = sync_domain_db(paths, config).await;
        log::info!("Synced domains. Exiting");
        return r
    }

    
    match args.name {
        Some(n) => find_and_print_domains(n, paths, config)?,
        None => ()
    };

    Ok(())
}
