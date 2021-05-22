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
    let proxy_config = match *proxy {
        Proxy::Direct => "DIRECT".to_owned(),
        Proxy::Socks(ref addr) => format!("SOCKS {}", build_addr(&addr)),
        Proxy::Socks4(ref addr) => format!("SOCKS4 {}", build_addr(&addr)),
        Proxy::Socks5(ref addr) => format!("SOCKS5 {}", build_addr(&addr)),
        Proxy::Http(ref addr) => format!("HTTP {}", build_addr(&addr)),
        Proxy::Https(ref addr) => format!("HTTPS {}", build_addr(&addr)),
    };
    proxy_config
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
