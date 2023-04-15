# Purpose

Considering that OpenAI's accessibility is not always user-friendly in certain regions, the need for a tool to forward API requests has arisen. Utilizing existing services could pose a significant risk of token exploitation, prompting the development of an in-house solution instead

# Solution 

Our using Hyper as the proxy to forward API requests. When we receive a request from the user, we replace the Host information and proceed to forward the request.

# Require

Compiling requires a Rust 

# Usage

```
Simple http proxy

Usage: x2x4-proxy [OPTIONS] --target <TARGET>

Options:
  -t, --target <TARGET>  which target you want to proxy
  -s, --schema <SCHEMA>  target http or https, default is https [default: https]
  -l, --listen <LISTEN>  ip to listen, default is 127.0.0.1 [default: 127.0.0.1]
  -p, --port <PORT>      port to listen, default is 3000 [default: 3000]
  -h, --help             Print help
  -V, --version          Print version
```

# Example

Start a simple proxy on http://127.0.0.1:3000 to https://api.somesite.com

```
x2x4-proxy -t api.somesite.com
```