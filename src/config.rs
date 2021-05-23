pub use concrete::Config;
use serde::Deserialize;

type ConfigResult<T> = Result<T, String>;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Address {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "UPPERCASE", deny_unknown_fields)]
pub enum Proxy {
    Direct,
    Socks(Address),
    Socks4(Address),
    Socks5(Address),
    Http(Address),
    Https(Address),
}

mod raw {
    use super::{ConfigResult, Proxy};
    use serde::Deserialize;
    use std::collections::HashMap;
    use toml::from_str;

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Rule {
        pub proxies: Vec<String>,
        pub allowed_hosts: Option<Vec<String>>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Config {
        pub proxies: HashMap<String, Proxy>,
        pub rules: Vec<Rule>,
    }

    impl Config {
        pub fn parse(content: &str) -> ConfigResult<Self> {
            from_str(content).map_err(|e| e.to_string())
        }
    }
}

mod concrete {
    use super::raw::Config as RawConfig;
    use super::{ConfigResult, Proxy};
    use std::collections::HashMap;
    use std::sync::Arc;

    #[derive(Debug)]
    pub struct Rule {
        pub proxies: Vec<Arc<Proxy>>,
        pub allowed_hosts: Option<Vec<String>>,
    }

    #[derive(Debug)]
    pub struct Config {
        pub proxies: HashMap<String, Arc<Proxy>>,
        pub rules: Vec<Rule>,
    }

    impl Config {
        pub fn from(raw_config: RawConfig) -> ConfigResult<Self> {
            let proxies: HashMap<_, _> = raw_config
                .proxies
                .into_iter()
                .map(|(name, proxy)| (name, Arc::new(proxy)))
                .collect();
            let rules = raw_config
                .rules
                .iter()
                .enumerate()
                .map(|(i, rule)| {
                    rule.proxies
                        .iter()
                        .map(|proxy_name| {
                            let proxies: Result<_, _> = proxies
                                .get(proxy_name)
                                .ok_or(format!(
                                    "Proxy \"{}\" referenced by rule #{} not found",
                                    proxy_name,
                                    i + 1,
                                ))
                                .map(|proxy| proxy.clone());
                            proxies
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map(|proxies| Rule {
                            proxies,
                            allowed_hosts: rule.allowed_hosts.clone(),
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Self { proxies, rules })
        }
    }
}

pub fn parse(config: &str) -> ConfigResult<concrete::Config> {
    let config = raw::Config::parse(&config)?;
    let config = concrete::Config::from(config)?;
    Ok(config)
}
