use crate::config::{Address, Config, Proxy};
use dynfmt::{Format, SimpleCurlyFormat};
use serde::Serialize;
use std::sync::Arc;

/// The PAC file template.
static PAC_TEMPLATE: &str = include_str!("template.pac");

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Rule {
    proxies: String,
    allowed_hosts: Option<Vec<String>>,
}

fn build_addr(addr: &Address) -> String {
    format!("{}:{}", addr.host, addr.port)
}

fn build_proxy(proxy: Arc<Proxy>) -> String {
    let (name, addr) = match *proxy {
        Proxy::Direct => ("DIRECT", None),
        Proxy::Socks(ref addr) => ("SOCKS", Some(addr)),
        Proxy::Socks4(ref addr) => ("SOCKS4", Some(addr)),
        Proxy::Socks5(ref addr) => ("SOCKS5", Some(addr)),
        Proxy::Http(ref addr) => ("HTTP", Some(addr)),
        Proxy::Https(ref addr) => ("HTTPS", Some(addr)),
    };
    addr.map(|addr| format!("{} {}", name, build_addr(addr)))
        .unwrap_or_else(|| name.to_owned())
}

fn build_rules(config: &Config) -> Vec<Rule> {
    let rules = config
        .rules
        .iter()
        .map(|rule| {
            let proxies = rule
                .proxies
                .iter()
                .map(|proxy| build_proxy(proxy.clone()))
                .collect::<Vec<_>>()
                .join("; ");
            Rule {
                proxies,
                allowed_hosts: rule.allowed_hosts.clone(),
            }
        })
        .collect::<Vec<_>>();
    rules
}

pub fn render(config: Config) -> String {
    let rules = build_rules(&config);
    let formatted = SimpleCurlyFormat.format(PAC_TEMPLATE, [rules]).unwrap();
    formatted.into_owned()
}
