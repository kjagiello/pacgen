# pacgen - Proxy Auto-Config (PAC) generator

Easily create a PAC file from a TOML config.

## Usage

### Using the Docker image

You can generate a PAC file from a TOML file using the latest stable pacgen Docker image:

```shell
docker run --rm -it -v $(pwd)/your-proxy.toml:/proxy.toml pacgen proxy.toml
```

You can also serve this file (the HTTP server binds by default at `127.0.0.1:8000`):

```shell
docker run --rm -it -v $(pwd)/your-proxy.toml:/proxy.toml pacgen -s proxy.toml
```

### CLI documentation

```
USAGE:
    pacgen [FLAGS] [OPTIONS] <CONFIG>

FLAGS:
        --help       Prints help information
    -s, --serve      Serves the generated PAC file
    -V, --version    Prints version information

OPTIONS:
    -h <host>        Host to bind the PAC server to
    -p <port>        Port to bind the PAC server to

ARGS:
    <CONFIG>    Path to the config file to use (- for STDIN).
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
