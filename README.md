# pacgen - Proxy Auto-Config (PAC) generator

Generates a [PAC file][pac-def] from a TOML config.

[pac-def]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Proxy_servers_and_tunneling/Proxy_Auto-Configuration_PAC_file

## Features

- Supported rules:
    - Forwarding traffic to a proxy based on hostnames (with support for globs)
- Built-in server for serving the generated PAC file

## Usage

### Using the Docker image

#### Generate a PAC file from a TOML file

```shell
docker run \
    --rm -it \
    -v $(pwd)/your-proxy.toml:/proxy.toml \
    ghcr.io/kjagiello/pacgen:latest \
    proxy.toml
```

#### Generate and serve the PAC file

The HTTP server binds by default at `127.0.0.1:8080`.

```shell
docker run \
    --rm -it \
    -v $(pwd)/your-proxy.toml:/proxy.toml \
    -p 127.0.0.1:8080:8080 \
    ghcr.io/kjagiello/pacgen:latest \
    -h 0.0.0.0
    -s proxy.toml
```

## Example config

```toml
[proxies]
secret-tunnel = { type = "SOCKS", host = "10.0.0.1", port = 1080 }
corporate-tunnel = { type = "HTTPS", host = "10.0.0.2", port = 443 }
direct = { type = "DIRECT" }

[[rules]]
# A fail-close mechanism. If the user is visiting "*.fbi.gov", require that the
# traffic flows through the "secret-tunnel" proxy.
proxy = ["secret-tunnel"]
allowed_hosts = ["*.fbi.gov"]

[[rules]]
# A fail-open mechanism. For any traffic, try to route it through the
# "corporate-tunnel" proxy and in case of the failure, let the traffic bypass
# the proxy.
proxy = ["corporate-tunnel", "direct"]
```

## The config format

- [[proxies]](#proxies) – Defines a proxy.
    - [type](#the-type-field) – The type of the proxy.
    - [host](#the-host-field) – The host of the proxy.
    - [port](#the-port-field) – The port of the proxy.
- [[[rule]]](#rule) – Defines a routing rule.
    - [proxy](#the-proxy-field) – The proxies to be used by the rule.
    - [allowed_hosts](#the-allowed_hosts-field) – The hosts that trigger the rule.

### Proxies

The first section in the config specifies the proxies that are available to use
by the rules. Each proxy has to be given name that will be used to reference
them from the rules.

```toml
[proxies]
proxy-a = { type = "SOCKS", host = "10.0.0.1", port = 1080 }
proxy-b = { type = "SOCKS", host = "10.0.0.2", port = 1081 }
```

#### The `type` field

Available values:

- `DIRECT`
- `SOCKS`
- `SOCKS4`
- `SOCKS5`
- `HTTP`
- `HTTPS`

The `DIRECT` type is a special one, because it instructs the traffic to not
flow through any proxy and does thus not require `host` and `proper` fields to
be specified.

#### The `host` field

Specifies which host the proxy is available at.

#### The `port` field

Specifies which port the proxy is available at.

### Rule

List of rules that are evaluated in the order they appear in in the config. In the example below,
the traffic to `*.evil.corp` will be routed through the `corporate-tunnel`
proxy, while all other traffic will go straight to the target.

```toml
[[rules]]
proxy = ["corporate-tunnel"]
allowed_hosts = ["*.evil.corp"]

[[rules]]
proxy = ["direct"]
```

#### The `proxy` field

A list of the proxy identifieries defined in [[proxies]](#proxies).

#### The `allowed_hosts` field

A list of the hosts that the rule should be triggered for. Every entry in this
list supports following globs:

- `?` – any single character. Example: `?.evil.corp` (will match `d.evil.corp`,
  but not `an.evil.corp`)
- `*` – any number of characters. Example: `*.evil.corp`

### CLI documentation

```
USAGE:
    pacgen [FLAGS] [OPTIONS] <CONFIG>

FLAGS:
        --help       Prints help information
    -s, --serve      Serves the generated PAC file
    -V, --version    Prints version information

OPTIONS:
    -h <host>        Host to bind the PAC server at [default: 127.0.0.1]
    -p <port>        Port to bind the PAC server at [default: 8080]

ARGS:
    <CONFIG>    Path to the config file to use (- for STDIN).
```

## Configuring PAC

### macOS

1. Open **System Preferences**
2. Go to **Network**
3. Choose the active network in the list to the left
4. Open **Advanced...**
5. Go to the **Proxies** tab.
6. Activate **Automatic Proxy Configuration**.
7. Set the URL to http://localhost:8080.
8. Press **Ok** and then **Apply**.

<p align="center">
    <img
        src="https://user-images.githubusercontent.com/74944/119229679-6b144880-bb19-11eb-9d2f-fa9388e4c6f4.png"
        alt="Automatic Proxy Configuration screenshot"
    />
</p>
